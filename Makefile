build:
	cargo build

run:
	RUST_LOG=info cargo run -r

run-logged:
	RUST_LOG=debug cargo run -r

precommit:
	cargo fmt
	cargo clippy --all-features --all-targets