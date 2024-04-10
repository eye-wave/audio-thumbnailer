#!make
.PHONY: install format lint

install:
	cargo build --release
	sudo cp target/release/audio-thumbnailer /usr/bin/mp3-thumbnailer
	sudo cp mp3-thumbnailer.thumbnailer /usr/share/thumbnailers

format:
	cargo fmt

lint:
	cargo clippy
