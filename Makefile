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
	rm test/*.jpg -f
	cargo run -- -i test/song.mp3 -o test/thumb.jpg -p triangle -a crop
	time cargo run -- -i test/file.mp3 -o test/thumb-file.jpg
	time cargo run -- -i test/wavetable.wav -o test/thumb-table.jpg
