run: build
	cd demo && python3 -m http.server

build:
	cargo build --release --target=wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/rustycheckers.wasm demo/

clean:
	rm -rf target
	rm demo/rustycheckers.wasm