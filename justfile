default: run

test:
    cargo test

watch:
    watchexec -c -w src -- just test

run *ARGS :
    cargo run -- {{ARGS}}

dry-publish:
    cargo publish --dry-run
