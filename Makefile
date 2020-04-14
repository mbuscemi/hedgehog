clean:
	rm static/hedgehog-frontend.js
	rm static/hedgehog-frontend.wasm

process:
	cd frontend && cargo web build --release
	cp frontend/target/wasm32-unknown-unknown/release/hedgehog-frontend.js static/
	cp frontend/target/wasm32-unknown-unknown/release/hedgehog-frontend.wasm static/

build:
	cargo build

run:
	cargo run
