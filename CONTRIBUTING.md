# Contributing to Fusabi TUI Runtime

Thank you for your interest in contributing! This document provides guidelines and information for contributors.

## Code of Conduct

Be respectful and inclusive. We're all here to build great software together.

## Getting Started

### Development Setup

```bash
# Clone the repository
git clone https://github.com/fusabi-lang/fusabi-tui-runtime.git
cd fusabi-tui-runtime

# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Run examples
cargo run --example basic_app -p fusabi-tui-engine
```

### Project Structure

```
fusabi-tui-runtime/
├── crates/
│   ├── fusabi-tui-core/     # Core primitives (Cell, Buffer, Layout, Style)
│   ├── fusabi-tui-render/   # Renderer abstraction
│   ├── fusabi-tui-widgets/  # Widget library
│   ├── fusabi-tui-engine/   # Hot reload engine
│   └── fusabi-tui-scarab/   # Shared memory backend
├── fsx/                      # Fusabi script library
├── docs/                     # Documentation
└── .github/workflows/        # CI/CD
```

## Making Changes

### Branch Naming

- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation updates
- `refactor/description` - Code refactoring

### Commit Messages

Follow conventional commits:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `test`: Tests
- `refactor`: Code refactoring
- `perf`: Performance improvement
- `chore`: Maintenance

Examples:
```
feat(widgets): add BarChart widget
fix(engine): handle empty file reload
docs(readme): update installation instructions
test(core): add buffer merge tests
```

### Code Style

We use standard Rust formatting and linting:

```bash
# Format code
cargo fmt --all

# Run lints
cargo clippy --workspace --all-targets

# Check documentation
cargo doc --workspace --no-deps
```

### Testing Requirements

- All new features must have tests
- All bug fixes should include a regression test
- Run the full test suite before submitting:

```bash
cargo test --workspace
```

### Documentation Requirements

- Public APIs must have rustdoc comments
- Include examples in doc comments where helpful
- Update relevant README files

## Pull Request Process

1. **Fork and branch** from `main`
2. **Make changes** following the guidelines above
3. **Test** your changes locally
4. **Push** to your fork
5. **Open PR** with clear description

### PR Checklist

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] Code is formatted (`cargo fmt`)
- [ ] Clippy passes (`cargo clippy`)
- [ ] Documentation updated if needed
- [ ] Commit messages follow convention

### Review Process

1. Maintainer will review within a few days
2. Address any feedback
3. Once approved, maintainer will merge

## Crate-Specific Guidelines

### fusabi-tui-core

- Keep types simple and copyable where possible
- Minimize dependencies
- Maintain API stability

### fusabi-tui-widgets

- Follow existing widget patterns
- Include builder methods
- Add to the widget prelude

### fusabi-tui-render

- Renderers must implement the `Renderer` trait
- Keep backend-specific code behind features

### fusabi-tui-engine

- Hot reload changes need careful testing
- State management should be predictable

### fusabi-tui-scarab

- Shared memory changes need cross-process testing
- Maintain protocol compatibility

## Performance Considerations

- Profile before optimizing
- Avoid allocations in hot paths
- Use differential updates where possible
- Document performance characteristics

## Questions?

- Open an issue for questions
- Check existing issues and docs first
- Be specific about what you need help with

## License

Contributions are licensed under MIT OR Apache-2.0 (same as the project).
