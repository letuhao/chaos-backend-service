# PowerShell script to remove all test code from src directory
# This script removes #[cfg(test)] modules and inline tests from source files

$srcPath = "crates\actor-core\src"
$files = Get-ChildItem -Path $srcPath -Recurse -Filter "*.rs" | Where-Object { $_.FullName -notlike "*\tests\*" }

Write-Host "Found $($files.Count) Rust files in src directory"

foreach ($file in $files) {
    $content = Get-Content $file.FullName -Raw
    $originalLines = $content.Split("`n").Count
    
    # Remove #[cfg(test)] modules and their contents
    $content = $content -replace '(?s)#\[cfg\(test\)\].*?(?=\n\n|\n#[^\[\s]|\Z)', ''
    
    # Remove standalone #[test] functions
    $content = $content -replace '(?s)#\[test\].*?(?=\n\n|\n#[^\[\s]|\nfn\s+[^_]|\Z)', ''
    
    # Remove #[tokio::test] functions
    $content = $content -replace '(?s)#\[tokio::test\].*?(?=\n\n|\n#[^\[\s]|\nfn\s+[^_]|\Z)', ''
    
    # Clean up extra newlines
    $content = $content -replace '\n{3,}', "`n`n"
    
    # Remove trailing whitespace
    $content = $content.TrimEnd()
    
    $newLines = $content.Split("`n").Count
    $removedLines = $originalLines - $newLines
    
    if ($removedLines -gt 0) {
        Write-Host "Removed $removedLines lines from $($file.Name)"
        Set-Content $file.FullName -Value $content -NoNewline
    }
}

Write-Host "Test removal completed!"
