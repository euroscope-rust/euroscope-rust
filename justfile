set shell := ["powershell.exe", "-NoLogo", "-Command"]

# List available commands
default:
    just --list

alias f := fmt
# Auto format code
fmt:
    Get-ChildItem -Recurse -Include *.cpp, *.h | ForEach-Object { clang-format -style=Mozilla -i $_.FullName }
    cargo +nightly fmt

alias l := lint
# Lint code
lint:
    cargo clippy

alias b := build
# Build (debug)
build:
    cargo build
alias br := build-release
# Build (release)
build-release:
    cargo build --release

alias t := test
# Run tests
test:
    cargo test

docs:
    cargo doc

# Cleanup rust build directory
clean:
    cargo clean
