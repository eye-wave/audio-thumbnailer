#!make
.PHONY: install uninstall format lint test clean

install:
	cargo build --release
	sudo cp target/release/audio-thumbnailer /usr/bin/
	sudo cp audio.thumbnailer /usr/share/thumbnailers

uninstall:
	sudo rm /usr/bin/audio-thumbnailer
	sudo rm /usr/share/thumbnailers/audio.thumbnailer

format:
	cargo fmt

lint:
	cargo clippy

test:
	rm -rf test/generated/*
	mkdir -p test/generated

	find test -not -path '*generated*' -type f | \
	while read input; do \
		cargo run -- -i "$$input" -o "test/generated/$$(basename "$$input").jpg"; \
	done

	cargo run -- -i test/1_fish.mp3 -o test/generated/thumbnail
	mv test/generated/thumbnail test/generated/thumbnail.jpg

	cargo test

clean:
	cargo clean
	rm test/*.jpg -f
