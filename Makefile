pre-commit:
	cargo machete --fix
	cargo features prune
	cargo fmt
	cargo clippy --fix
