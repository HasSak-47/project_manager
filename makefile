
build_client:
	@cd ./crates/client && cargo rustc -- -Awarnings && cargo build

run:
	@cargo run

impl_util:
	@cargo build --release
	@mv target/release/project_manager_client ~/.cmd/project_manager

