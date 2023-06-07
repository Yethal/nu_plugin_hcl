set shell := ["nu", "-c"]

# List available recipes
default:
    @just --list

# Build package
build:
    @cargo build

# Create cargo package
package:
    @cargo package

# Publish package to crates.io
publish:
    @cargo publish
