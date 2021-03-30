watch:
	cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build"

watch-frontend:
	cd www && npm start

build:
	cargo build
	wasm-pack build

build-release:
	cargo build --release
	wasm-pack build --release
	ls -lh pkg

install:
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
	cd www && npm install
