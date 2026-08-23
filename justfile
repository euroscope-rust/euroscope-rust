set shell := ["powershell.exe", "-NoLogo", "-Command"]

# List available commands
default:
    just --list

alias f := fmt
# Auto format code
fmt:
    Get-ChildItem -Recurse -Include *.cpp, *.h | ForEach-Object { clang-format -style=Mozilla -i $_.FullName }
    cargo +nightly fmt
[private]
ci-fmt:
    Get-ChildItem -Recurse -Include *.cpp, *.h | ForEach-Object { clang-format -style=Mozilla --dry-run --Werror $_.FullName }
    cargo +nightly fmt --check

alias l := lint
# Lint code
lint:
    cargo clippy --workspace --all-targets --all-features
[private]
ci-lint:
    $env:RUSTFLAGS = "-Dwarnings"; just lint

alias b := build
# Build (debug)
build:
    cargo build --workspace --all-features
alias br := build-release
# Build (release)
build-release:
    cargo build --workspace --all-features --release
[private]
ci-build:
    $env:RUSTFLAGS = "-Dwarnings"; just build-release

alias t := test
# Run tests
test:
    $env:EUROSCOPE_PLUGIN_DELAYLOAD = "1"; cargo test --workspace --all-features
    $env:EUROSCOPE_PLUGIN_DELAYLOAD = "1"; cargo test --workspace --all-features --doc
[private]
ci-test:
    $env:RUSTFLAGS = "-Dwarnings"; just test

docs:
    cargo doc

# Cleanup rust build directory
clean:
    cargo clean
