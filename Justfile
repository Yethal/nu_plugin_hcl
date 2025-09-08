set shell := ["nu", "-c"]

# List available recipes
default:
    @just --list

# Build package
build: bump-version
    @cargo build

# Create cargo package
package:
    @cargo package

# Publish package to crates.io
publish:
    @cargo publish

# Bump version in Cargo.toml to match Nushell
bump-version:
    #!/usr/bin/env nu
    let nushell_version = (http get https://api.github.com/repos/nushell/nushell/releases/latest).tag_name
    let plugin_version = (open Cargo.toml).package.version
    print $"Bumping version to ($nushell_version)"
    open Cargo.toml --raw
    | str replace -a $plugin_version $nushell_version
    | save -f Cargo.toml
    

# Tag release
tag: build
    #!/usr/bin/env nu
    let plugin_version = (open Cargo.toml).package.version
    let message = $"Bump version to ($plugin_version)"
    git add .
    git commit -m $message
    git tag -a $plugin_version -m $message
    git push --tags
