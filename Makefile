default:
	@make lint
	@make format
	@make doc

lint:
	@cargo clippy --all-features --tests

format:
	@cargo +nightly fmt --all

doc:
	@cargo doc --no-deps --all-features
