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

	cargo build

	find test -not -path '*generated*' -type f | \
	while read input; do \
		./target/debug/audio-thumbnailer -i "$$input" -o "test/generated/$$(basename "$$input").jpg"; \
	done

	./target/debug/audio-thumbnailer -i test/1_fish.mp3 -o test/generated/thumbnail
	mv test/generated/thumbnail test/generated/thumbnail.jpg

	cargo test

clean:
	cargo clean
	rm test/*.jpg -f
