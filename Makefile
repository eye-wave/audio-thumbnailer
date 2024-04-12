#!make
.PHONY: install format lint test

install:
	cargo build --release
	sudo cp target/release/audio-thumbnailer /usr/bin/
	sudo cp audio.thumbnailer /usr/share/thumbnailers

format:
	cargo fmt

lint:
	cargo clippy

test:
	rm test/**/*.jpg -f
	mkdir -p test/thumb

	cargo test
