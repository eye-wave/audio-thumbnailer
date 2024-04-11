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
    rm test/*.jpg -f
    cargo run -- -i test/song.mp3 -o test/thumb.jpg -p triangle -a crop
    time cargo run -- -i test/file.mp3 -o test/thumb-file.jpg
    time cargo run -- -i test/wavetable.wav -o test/thumb-table.jpg

clean:
    cargo clean
    rm test/*.jpg -f
