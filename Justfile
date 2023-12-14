set dotenv-load := true

help:
    @just --list --unsorted

build:
    cargo build
alias b := build

run *args:
    cargo run {{args}}
alias r := run

release:
    cargo build --release

install:
    cargo install --path .

bootstrap:
    cargo install cargo-edit

test *args:
    cargo test {{args}}

check:
    cargo check
alias c := check

fix:
    cargo clippy --fix

# Bump version. level=major,minor,patch
version level:
    git diff-index --exit-code HEAD > /dev/null || ! echo You have untracked changes. Commit your changes before bumping the version.
    cargo set-version --bump {{level}}
    cargo update # This bumps Cargo.lock
    VERSION=$(rg  "version = \"([0-9.]+)\"" -or '$1' Cargo.toml | head -n1) && \
        git commit -am "Bump version {{level}} to $VERSION" && \
        git tag v$VERSION && \
        git push origin v$VERSION
    git push

publish:
    cargo publish

patch: test
    just version patch
    just publish

doc:
    cargo doc --no-deps --open

ci:
    cargo test --profile ci --no-run
    cargo test --profile ci

transform:
    req https://raw.githubusercontent.com/plaid/plaid-openapi/master/2020-09-14.yml > openapi.yaml
    read
    cd transform && cargo run -- ../openapi.yaml ../transform.yaml

generate:
    checkexec transform.yaml -- just transform
    libninja gen -lrust plaid transform.yaml
