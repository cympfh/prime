all: build-rust build-svelte
	rm -rf docs
	cp -R public docs

dev:
	npm run dev

build-svelte:
	npm run build

build-rust:
	cargo build

test:
	npm run check
