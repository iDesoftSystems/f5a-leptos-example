start:
    trunk serve

check:
    cargo clippy --all-targets --all-features -- -D warnings
