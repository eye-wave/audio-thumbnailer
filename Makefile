#!make
.PHONY: install format lint test

install:
	cargo build --release
	sudo cp target/release/audio-thumbnailer /usr/bin/mp3-thumbnailer
	sudo cp mp3-thumbnailer.thumbnailer /usr/share/thumbnailers

format:
	cargo fmt

lint:
	cargo clippy

test:
	rm test/thumb.jpg
	cargo run -- -i test/song.mp3 -o test/thumb.jpg -p triangle -a crop
	sxiv test/thumb.jpg
