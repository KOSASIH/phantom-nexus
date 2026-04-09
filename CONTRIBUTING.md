# Contributing to Phantom Nexus

Thank you for your interest in contributing to Phantom Nexus! 👻

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/phantom-nexus.git`
3. Create a branch: `git checkout -b feature/your-feature`
4. Make changes and commit: `git commit -m "feat: description"`
5. Push and create a Pull Request

## Development Setup

### Prerequisites
- Rust 1.75+ (`rustup install stable`)
- Node.js 20+ (for frontend)
- Python 3.11+ (for AI modules)

### Build & Test
```bash
cargo build --all
cargo test --all
cargo fmt --all
cargo clippy --all-targets
```

## Commit Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation
- `refactor:` Code refactoring
- `test:` Adding tests
- `chore:` Maintenance

## Code Style
- Rust: `cargo fmt` (rustfmt defaults)
- TypeScript: Prettier + ESLint
- Python: Black + isort

## Security
Report security vulnerabilities privately via GitHub Security Advisories.

## License
By contributing, you agree that your contributions will be licensed under the MIT License.
