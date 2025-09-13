param(
    [Parameter(Mandatory = $true)][string]$InputPath,
    [Parameter(Mandatory = $true)][string]$OutDir,
    [ValidateSet('chunk', 'per-file', 'size', 'group')][string]$Mode = 'chunk',
    [int]$ChunkSize = 120,
    [long]$MaxBytes = 10MB,
    [switch]$GenerateManifest,
    [string]$GroupsFile,
    [switch]$Gzip
)

# 中文: 将 llvm-cov 导出的 coverage.json 拆分为多个较小文件（支持分块/按文件/按大小）
# Tiếng Việt: Tách coverage.json (xuất từ llvm-cov) thành nhiều file nhỏ (theo chunk/theo file/theo kích thước)

$ErrorActionPreference = 'Stop'

function New-DirIfMissing {
    param([string]$Path)
    if (-not (Test-Path $Path)) {
        New-Item -ItemType Directory -Path $Path | Out-Null
    }
}

function Get-CoverageJson {
    param([string]$Path)
    if (-not (Test-Path $Path)) { throw "Input not found: $Path" }
    $json = Get-Content $Path -Raw
    if ([string]::IsNullOrWhiteSpace($json)) { throw "Input is empty: $Path" }
    $cov = $json | ConvertFrom-Json -Depth 100
    if (-not $cov -or -not $cov.data -or $cov.data.Count -eq 0) {
        throw "Invalid coverage JSON: missing data[]"
    }
    return $cov
}

function New-ExportObject {
    param(
        $CovRoot,
        $FilesSubset,
        $FunctionsSubset
    )
    $chunkData = [PSCustomObject]@{
        files     = $FilesSubset
        functions = $FunctionsSubset
        totals    = $null
    }
    $export = [PSCustomObject]@{
        type    = $CovRoot.type
        version = $CovRoot.version
        data    = @($chunkData)
    }
    return $export
}

function Out-JsonFile {
    param([object]$Obj, [string]$Path, [switch]$Compress)
    $json = $Obj | ConvertTo-Json -Depth 100
    if ($Compress) {
        $gzPath = "$Path.gz"
        $bytes = [System.Text.Encoding]::UTF8.GetBytes($json)
        $fs = [System.IO.File]::Create($gzPath)
        try {
            $gs = New-Object System.IO.Compression.GZipStream($fs, [System.IO.Compression.CompressionLevel]::Optimal)
            $gs.Write($bytes, 0, $bytes.Length)
            $gs.Close()
        } finally { $fs.Close() }
        return $gzPath
    } else {
        $json | Out-File -Encoding UTF8 $Path
        return $Path
    }
}

function Select-FunctionsByFiles {
    param($FunctionsAll, [System.Collections.Generic.HashSet[string]]$FileNameSet)
    if ($null -eq $FunctionsAll) { return @() }
    return $FunctionsAll | Where-Object {
        $null -ne $_.filenames -and ($_.filenames | Where-Object { $FileNameSet.Contains($_) }).Count -gt 0
    }
}

function ConvertTo-SafePathPart {
    param([string]$p)
    return $p.Replace(':','_').Replace('/','_').Replace('\','_')
}

function Measure-JsonBytes {
    param($Obj)
    $s = $Obj | ConvertTo-Json -Depth 100
    return [System.Text.Encoding]::UTF8.GetByteCount($s)
}

# Load input
New-DirIfMissing -Path $OutDir
$cov = Get-CoverageJson -Path $InputPath
$entry = $cov.data[0]
$files = @($entry.files)
$functions = @($entry.functions)

# Build manifest info
$manifest = [ordered]@{
    source           = (Resolve-Path $InputPath).Path
    mode             = $Mode
    chunkSize        = $ChunkSize
    maxBytes         = $MaxBytes
    createdAt        = (Get-Date).ToString("s")
    type             = $cov.type
    version          = $cov.version
    totalFiles       = ($files | Measure-Object).Count
    totalFunctions   = ($(if ($functions) { ($functions | Measure-Object).Count } else { 0 }))
    parts            = @()
}

switch ($Mode) {
    'chunk' {
        for ($i = 0; $i -lt $files.Count; $i += $ChunkSize) {
            $end = [Math]::Min($i + $ChunkSize - 1, $files.Count - 1)
            $chunkFiles = $files[$i..$end]
            $nameSet = New-Object 'System.Collections.Generic.HashSet[string]'
            foreach ($f in $chunkFiles) { [void]$nameSet.Add($f.filename) }
            $chunkFunctions = Select-FunctionsByFiles -FunctionsAll $functions -FileNameSet $nameSet
            $export = New-ExportObject -CovRoot $cov -FilesSubset $chunkFiles -FunctionsSubset $chunkFunctions
            $index = [int]([math]::Floor($i / $ChunkSize)) + 1
            $outFile = Join-Path $OutDir ("coverage_chunk_{0:D4}.json" -f $index)
            $written = Out-JsonFile -Obj $export -Path $outFile -Compress:$Gzip
            $manifest.parts += [ordered]@{
                file          = (Resolve-Path $written).Path
                numFiles      = $chunkFiles.Count
                numFunctions  = $chunkFunctions.Count
                approxBytes   = (Get-Item $written).Length
            }
        }
    }
    'per-file' {
        foreach ($f in $files) {
            $set = New-Object 'System.Collections.Generic.HashSet[string]'
            [void]$set.Add($f.filename)
            $ffuncs = Select-FunctionsByFiles -FunctionsAll $functions -FileNameSet $set
            $export = New-ExportObject -CovRoot $cov -FilesSubset @($f) -FunctionsSubset $ffuncs
            $outFile = Join-Path $OutDir ("file_" + (ConvertTo-SafePathPart $f.filename) + ".json")
            $written = Out-JsonFile -Obj $export -Path $outFile -Compress:$Gzip
            $manifest.parts += [ordered]@{
                file          = (Resolve-Path $written).Path
                numFiles      = 1
                numFunctions  = $ffuncs.Count
                approxBytes   = (Get-Item $written).Length
            }
        }
    }
    'size' {
        # Pack files to stay under MaxBytes per part (best-effort)
        $currentFiles = New-Object System.Collections.Generic.List[object]
        $currentNames = New-Object 'System.Collections.Generic.HashSet[string]'
        $currentBytes = 0L
        $partIndex = 0

        function Invoke-FlushPart {
            param()
            if ($currentFiles.Count -eq 0) { return }
            $partIndex++
            $chunkFunctions = Select-FunctionsByFiles -FunctionsAll $functions -FileNameSet $currentNames
            $export = New-ExportObject -CovRoot $cov -FilesSubset $currentFiles -FunctionsSubset $chunkFunctions
            $outFile = Join-Path $OutDir ("coverage_part_{0:D4}.json" -f $partIndex)
            $written = Out-JsonFile -Obj $export -Path $outFile -Compress:$Gzip
            $manifest.parts += [ordered]@{
                file          = (Resolve-Path $written).Path
                numFiles      = $currentFiles.Count
                numFunctions  = $chunkFunctions.Count
                approxBytes   = (Get-Item $written).Length
            }
            $script:currentFiles = New-Object System.Collections.Generic.List[object]
            $script:currentNames = New-Object 'System.Collections.Generic.HashSet[string]'
            $script:currentBytes = 0L
        }

        foreach ($f in $files) {
            # Estimate JSON bytes for this single file export skeleton to avoid overshoot
            $estimate = Measure-JsonBytes -Obj ([pscustomobject]@{ files=@($f); functions=@(); totals=$null })
            if ($estimate -gt $MaxBytes) {
                # Single file exceeds max; write it alone
                if ($currentFiles.Count -gt 0) { Invoke-FlushPart }
                $soloSet = New-Object 'System.Collections.Generic.HashSet[string]'
                [void]$soloSet.Add($f.filename)
                $soloFuncs = Select-FunctionsByFiles -FunctionsAll $functions -FileNameSet $soloSet
                $exportSolo = New-ExportObject -CovRoot $cov -FilesSubset @($f) -FunctionsSubset $soloFuncs
                $outSolo = Join-Path $OutDir ("coverage_part_{0:D4}.json" -f (++$partIndex))
                $writtenSolo = Out-JsonFile -Obj $exportSolo -Path $outSolo -Compress:$Gzip
                $manifest.parts += [ordered]@{
                    file          = (Resolve-Path $writtenSolo).Path
                    numFiles      = 1
                    numFunctions  = $soloFuncs.Count
                    approxBytes   = (Get-Item $writtenSolo).Length
                }
                continue
            }

            if (($currentBytes + $estimate) -gt $MaxBytes -and $currentFiles.Count -gt 0) {
                Invoke-FlushPart
            }
            $currentFiles.Add($f)
            [void]$currentNames.Add($f.filename)
            $currentBytes += $estimate
        }
        if ($currentFiles.Count -gt 0) { Invoke-FlushPart }
    }
    'group' {
        if (-not $GroupsFile) { throw "Mode 'group' requires -GroupsFile <path to JSON mapping>" }
        if (-not (Test-Path $GroupsFile)) { throw "GroupsFile not found: $GroupsFile" }
        $groupsJson = Get-Content $GroupsFile -Raw | ConvertFrom-Json -Depth 50
        # Normalize groups to ordered hashtable of name -> patterns (array)
        $groups = @{}
        foreach ($prop in $groupsJson.PSObject.Properties) { $groups[$prop.Name] = @($prop.Value) }

        # Helper: returns true if path matches any glob in list
        function Test-MatchAny {
            param([string]$PathVal, [string[]]$Patterns)
            foreach ($pat in $Patterns) {
                $normPat = $pat.Replace('/', '\\')
                $normPath = $PathVal.Replace('/', '\\')
                # Convert simple glob ** -> * for -like best-effort; prefix/suffix wildcards
                $like = $normPat.Replace('**','*')
                if ($normPath -like $like) { return $true }
            }
            return $false
        }

        $unassigned = New-Object System.Collections.Generic.List[object]
        $unassignedNames = New-Object 'System.Collections.Generic.HashSet[string]'
        foreach ($f in $files) { $unassigned.Add($f); [void]$unassignedNames.Add($f.filename) }

        foreach ($gName in $groups.Keys) {
            $pats = $groups[$gName]
            $gFiles = @()
            foreach ($f in $files) {
                if (Test-MatchAny -PathVal $f.filename -Patterns $pats) { $gFiles += $f }
            }
            if ($gFiles.Count -eq 0) { continue }
            $nameSet = New-Object 'System.Collections.Generic.HashSet[string]'
            foreach ($f in $gFiles) { [void]$nameSet.Add($f.filename); [void]$unassignedNames.Remove($f.filename) }
            $gFuncs = Select-FunctionsByFiles -FunctionsAll $functions -FileNameSet $nameSet
            $export = New-ExportObject -CovRoot $cov -FilesSubset $gFiles -FunctionsSubset $gFuncs
            $safe = ConvertTo-SafePathPart $gName
            $outFile = Join-Path $OutDir ("group_" + $safe + ".json")
            $written = Out-JsonFile -Obj $export -Path $outFile -Compress:$Gzip
            $manifest.parts += [ordered]@{
                file          = (Resolve-Path $written).Path
                group         = $gName
                numFiles      = $gFiles.Count
                numFunctions  = $gFuncs.Count
                approxBytes   = (Get-Item $written).Length
            }
        }
        # Remainder
        $rest = @()
        foreach ($f in $files) { if ($unassignedNames.Contains($f.filename)) { $rest += $f } }
        if ($rest.Count -gt 0) {
            $setRest = New-Object 'System.Collections.Generic.HashSet[string]'
            foreach ($f in $rest) { [void]$setRest.Add($f.filename) }
            $restFuncs = Select-FunctionsByFiles -FunctionsAll $functions -FileNameSet $setRest
            $exportR = New-ExportObject -CovRoot $cov -FilesSubset $rest -FunctionsSubset $restFuncs
            $outR = Join-Path $OutDir ("group_rest.json")
            $writtenR = Out-JsonFile -Obj $exportR -Path $outR -Compress:$Gzip
            $manifest.parts += [ordered]@{
                file          = (Resolve-Path $writtenR).Path
                group         = "rest"
                numFiles      = $rest.Count
                numFunctions  = $restFuncs.Count
                approxBytes   = (Get-Item $writtenR).Length
            }
        }
    }
}

if ($GenerateManifest) {
    $manifestPath = Join-Path $OutDir 'manifest.json'
    ($manifest | ConvertTo-Json -Depth 100) | Out-File -Encoding UTF8 $manifestPath
}

Write-Host "Done. Output: $OutDir" -ForegroundColor Green

