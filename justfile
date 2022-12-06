default: run

test:
    cargo test

watch:
    watchexec -c -w src -- just test

watch-help:
    watchexec -c -w src -- cargo run -- --help

run *ARGS :
    cargo run -- {{ARGS}}

dry-publish:
    cargo publish --dry-run
