clean:
	cargo clean
	rm -rf ./www/dist

pack:
	wasm-pack build --target web
	cp ./pkg/snake_lets_go_bg.wasm ./www/public/snake_lets_go.wasm
