all: wasm

server: wasm
	python3 -m http.server 8080

wasm: ./pkg/concrete_wasm_test.js

./pkg/concrete_wasm_test.js: Cargo.toml Cargo.lock ./src/lib.rs
	wasm-pack build --target=web

.PHONY: all server wasm