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
	rm test/**/*.jpg -f
	mkdir -p test/thumb

	cargo test

clean:
    cargo clean
    rm test/*.jpg -f
