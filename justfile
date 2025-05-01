default:
    just --list

test-all:
    cargo +stable vet --locked
    cargo +stable deny --all-features --locked check
    cargo +stable fmt
    cargo +stable build --locked
    cargo +stable clippy --locked
    cargo +stable test --locked
