run:
	cargo run

build_client:
	cd crates/client
	make

impl_util:
	cargo build --release
	mv target/release/project_manager_client ~/.cmd/project_manager

