clean:
	cargo clean
	rm -rf ./www/dist

pack:
	rm -rf ./pkg
	wasm-pack build --target web
	cp ./pkg/snake_lets_go_bg.wasm ./www/public/snake_lets_go.wasm
	rm -rf ./www/node_modules
	cd www && yarn install
