# Aria Compiler (`ar-c`)

`ar-c` is the reference compiler for the Aria Agentic Runtime. It is a command-line tool that bundles TypeScript projects into a standardized `.aria` package.

## Core Functionality

The compiler performs several key tasks:

1.  **Parses TypeScript**: It uses SWC (`swc_core`) to parse TypeScript code, with specific support for decorators (`@tool`, `@agent`, etc.) used by the Aria SDK.
2.  **Extracts Metadata**: It traverses the Abstract Syntax Tree (AST) to identify decorated entities and extracts their metadata (name, description, parameters) into a `manifest.json` file. This manifest is a critical component used by the Aria Runtime's `PlanningEngine` and `ReflectionEngine`.
3.  **Transpiles to JavaScript**: The original TypeScript source for each decorated entity is transpiled into executable JavaScript.
4.  **Packages the Bundle**: It assembles the `manifest.json` and all transpiled JavaScript implementations into a single, portable `.aria` file (which is a standard ZIP archive).

This process creates a self-contained, deployable unit that the Aria Runtime can execute, allowing for a clean separation between the development environment and the execution environment.

## Usage

```bash
# Build the project in the current directory
arc build .
```

## Quick Start

### Install & Build

```bash
# Clone and build
git clone https://github.com/aria-dev/ar-c.git
cd ar-c
cargo build --release

# Binary will be at: target/release/arc
```

### Usage

```bash
# Create new project
./target/release/arc new my-project --template=basic
cd my-project

# Build TypeScript to .aria bundle (placeholder implementation)
./target/release/arc build ./src --output dist/myproject.aria

# Upload to runtime server
./target/release/arc upload dist/myproject.aria --server https://aria-server.com --auth-token <token>
```

## Architecture

The Arc compiler follows a modular, extensible architecture designed for multi-language support:

```
src/
├── main.rs              # CLI entry point with clap integration
├── cli/                 # Command implementations
│   ├── mod.rs          # CLI utilities and formatting
│   ├── new.rs          # Project scaffolding (COMPLETE)
│   ├── build.rs        # Build command (PLACEHOLDER)
│   └── upload.rs       # Bundle upload (COMPLETE)
├── compiler/           # Core compilation engine
│   ├── mod.rs          # Main compiler orchestration
│   └── typescript/     # TypeScript-specific compiler (PLACEHOLDER)
├── bundle/             # .aria bundle format handling
│   └── mod.rs          # Bundle creation, loading, validation
└── config/             # Configuration management
    └── mod.rs          # aria.toml parsing and templates
```
- **SWC Integration**: Replace regex-based parsing with proper AST
- **Dependency Tracing**: Complete import resolution and bundling
- **Watch Mode**: File watching for development workflow
- **Performance Optimization**: Incremental compilation and caching
- **Error Reporting**: Source-mapped error messages

## Development

### Project Templates

The `arc new` command supports three templates:

#### Basic Template
```bash
arc new myproject --template=basic
```
- Single `src/index.ts` with @tool example
- Basic aria.toml configuration
- Ready-to-build structure

#### SDK Template  
```bash
arc new myproject --template=sdk
```
- Organized `src/tools/` and `src/agents/` directories
- Multiple example implementations
- Advanced project structure

#### Advanced Template
```bash
arc new myproject --template=advanced
```
- Full SDK structure plus teams and workflows
- Complex agent coordination examples
- Production-ready organization

### Configuration (aria.toml)

```toml
[project]
name = "my-aria-app"
version = "0.1.0"
description = "An Aria agentic application"

[build]
target = "typescript"
output = "dist/myapp.aria"
source_dirs = ["src"]
exclude = ["node_modules", "dist"]

[runtime]
bun_version = "latest"
```

### Bundle Structure

Generated `.aria` bundles use ZIP format:

```
myapp.aria
├── manifest.json              # Tool/agent metadata
├── implementations/           # Executable JavaScript
│   ├── tools/
│   ├── agents/
│   └── teams/
├── package.json               # Runtime dependencies
└── metadata/
    └── build.json            # Build information
```

## Contributing

### Adding New Commands

1. Create command module in `src/cli/`
2. Add command to `src/main.rs` 
3. Implement async handler function
4. Add tests in `tests/integration/`

### Extending TypeScript Compiler

The TypeScript compiler in `src/compiler/typescript/` currently uses placeholder regex parsing. To implement real SWC integration:

1. Uncomment SWC dependencies in `Cargo.toml`
2. Replace regex parsing with AST visitor pattern
3. Implement proper decorator metadata extraction
4. Add dependency tracing and scope analysis

### Future DSL Support

The architecture is designed for easy DSL integration:

1. Create `src/compiler/dsl/` module
2. Implement `LanguageCompiler` trait  
3. Add DSL detection to `src/compiler/mod.rs`
4. Extend bundle format for DSL-specific features

## Performance

### Build Performance Targets
- Small projects (1-10 tools): <1s
- Medium projects (10-100 tools): <5s
- Large projects (100+ tools): <30s
- Incremental builds: <2s

### Binary Size
- Release binary: <50MB (optimized with LTO)
- Typical .aria bundle: <5MB

## License

MIT License - see [LICENSE](LICENSE) file.

## Links

- **Architecture Document**: [ARC.md](ARC.md) - Complete technical design
- **Development Plan**: [PLAN.md](PLAN.md) - Detailed implementation roadmap  
- **Runtime Integration**: Compatible with Aria Runtime Server
- **TypeScript SDK**: Uses `@aria/sdk` decorators