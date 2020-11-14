build: src
	wasm-pack build --target web

serve: build
	python -m http.server 8080 -d pkg
