start:
    trunk serve --port=8081

check:
    cargo clippy --all-targets --all-features -- -D warnings
