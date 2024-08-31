init:
	@rustup update
	@rustup update nightly
	@rustup target add wasm32-unknown-unknown --toolchain nightly
	
	# For getting list of the slow compiled crates:
	@cargo install cargo-machete && cargo machete