# 🚀 Development Guide - Chaos Backend Service

## 📋 Overview

This guide explains how to develop with the Chaos Backend Service project, including how to suppress warnings for examples, benches, and tests.

## 🔧 Warning Suppression

### Problem
When developing with examples, benches, and tests, Rust compiler shows many warnings like:
- `unused_variables`
- `unused_imports` 
- `dead_code`
- `unused_mut`

These warnings can be annoying during development and clutter the output.

### Solutions

#### 1. **Using Development Scripts (Recommended)**

We provide convenient scripts to suppress warnings:

```bash
# Windows
scripts\dev.bat check
scripts\dev.bat build
scripts\dev.bat test
scripts\dev.bat examples new_architecture_demo
scripts\dev.bat bench
scripts\dev.bat clean

# PowerShell
.\scripts\dev.ps1 check
.\scripts\dev.ps1 build
.\scripts\dev.ps1 test
.\scripts\dev.ps1 examples new_architecture_demo
.\scripts\dev.ps1 bench
.\scripts\dev.ps1 clean
```

#### 2. **Environment Variables**

Set `RUSTFLAGS` environment variable:

```bash
# Windows
set RUSTFLAGS=-A unused_variables -A unused_imports -A dead_code -A unused_mut -A unused_assignments

# Linux/Mac
export RUSTFLAGS="-A unused_variables -A unused_imports -A dead_code -A unused_mut -A unused_assignments"
```

#### 3. **VS Code Configuration**

The project includes `.vscode/settings.json` to configure rust-analyzer:

```json
{
    "rust-analyzer.check.extraArgs": [
        "--",
        "-A", "unused_variables",
        "-A", "unused_imports", 
        "-A", "dead_code",
        "-A", "unused_mut"
    ],
    "rust-analyzer.diagnostics.disabled": [
        "unused_variables",
        "unused_imports",
        "dead_code",
        "unused_mut"
    ]
}
```

#### 4. **Code-level Suppression**

Add `#![allow()]` attributes to specific files:

```rust
#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

// Your code here...
```

## 🏗️ Project Structure

```
chaos-backend-service/
├── crates/
│   ├── condition-core/          # Condition evaluation system
│   ├── actor-core/             # Actor system
│   └── shared/                 # Shared utilities
├── services/                   # Microservices
├── scripts/                    # Development scripts
│   ├── dev.bat                # Windows batch script
│   └── dev.ps1                # PowerShell script
├── .vscode/                   # VS Code configuration
├── .cargo/                    # Cargo configuration
└── rust-toolchain.toml        # Rust toolchain configuration
```

## 🚀 Quick Start

### 1. **Check Project**
```bash
scripts\dev.bat check
```

### 2. **Run Examples**
```bash
# List available examples
scripts\dev.bat examples

# Run specific example
scripts\dev.bat examples new_architecture_demo
scripts\dev.bat examples add_new_function_demo
```

### 3. **Run Tests**
```bash
scripts\dev.bat test
```

### 4. **Run Benchmarks**
```bash
scripts\dev.bat bench
```

### 5. **Clean Build**
```bash
scripts\dev.bat clean
```

## 🎯 Condition Core Development

### New Architecture Benefits

The Condition Core has been refactored to use a **plugin-based architecture**:

- ✅ **No Breaking Changes**: Adding functions doesn't break existing code
- ✅ **Loose Coupling**: Functions only depend on `ElementDataAccessor`
- ✅ **Scalable**: Add unlimited functions without trait hell
- ✅ **Testable**: Each function can be tested independently

### Adding New Functions

```rust
// 1. Create function struct
pub struct MyNewFunction;

#[async_trait::async_trait]
impl ElementConditionFunction for MyNewFunction {
    fn name(&self) -> &str { "my_new_function" }
    
    async fn evaluate(&self, params: &[ConditionParameter], context: &ConditionContext, data_accessor: &ElementDataAccessor) -> ConditionResult<bool> {
        // Your logic here
        Ok(true)
    }
}

// 2. Register function
registry.register_function(MyNewFunction);

// 3. Use function
let result = registry.execute_function("my_new_function", &params, &context).await?;
```

## 🔧 Development Tools

### VS Code Extensions
- **rust-analyzer**: Rust language server
- **CodeLLDB**: Debugging support
- **Better TOML**: TOML file support

### Useful Commands

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check dependencies
cargo tree

# Update dependencies
cargo update

# Build documentation
cargo doc --open
```

## 📚 Examples

### Available Examples

1. **`new_architecture_demo`**: Demonstrates the new plugin-based architecture
2. **`add_new_function_demo`**: Shows how easy it is to add new functions
3. **`element_conditions`**: Element condition function examples

### Running Examples

```bash
# Run all examples
cargo run --example new_architecture_demo
cargo run --example add_new_function_demo
cargo run --example element_conditions

# Or use the dev script
scripts\dev.bat examples new_architecture_demo
```

## 🐛 Troubleshooting

### Common Issues

1. **Warnings still showing**: Make sure you're using the dev script or have set `RUSTFLAGS`
2. **Compilation errors**: Check that all dependencies are installed
3. **VS Code not working**: Restart rust-analyzer or reload window

### Getting Help

- Check the project documentation in `docs/` folder
- Look at examples in `examples/` folder
- Check the architecture refactor guide: `crates/condition-core/ARCHITECTURE_REFACTOR.md`

## 🎉 Conclusion

With the new architecture and development tools, you can:

- ✅ Develop without annoying warnings
- ✅ Add new functions easily
- ✅ Test individual components
- ✅ Scale the system infinitely

Happy coding! 🚀
