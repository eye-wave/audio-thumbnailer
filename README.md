# Audio File Thumbnailer

This is a fast and lightweight audio file thumbnailer written in Rust. It generates thumbnails for audio files that can be used by file managers.

## Overview

- **Fast and Lightweight**: Thanks to rust programming language.
- **Customizable**: You can specify precisely how you want your thumbnail to look, by customizing ~~quality~~ , aspect ratio treatment, interpolation algorithm, waveform color, and waveform size.
- **Multipurpose**: It can generate album covers for audio files. Without embedded cover art it generates waveforms to represent the audio content visually.
- **Modular**: You can out-out of features you don't need

## Features
| feature_name | description | additional info |
|-|-|-|
| config_file | adds a option to customize the behavior of the app with a single config file | run the app with `--init-config` flag to create a example config file |
| colored_waveform | adds a fft algorithm to the waveform generation to color them based on frequency |
| 3d_wavetables | checks for .wav files if they are a wavetable file. if they are, generates a 3d view of them | useful when dealing with samples for synthesizers like [serum](https://xferrecords.com/products/serum) or [vital](https://vital.audio)

### Installation

Before using the thumbnailer, ensure that you have Rust installed on your system. You can install Rust from [here](https://www.rust-lang.org/tools/install).

```bash
# Clone the repository
git clone https://github.com/eye-wave/audio-thumbnailer

# Navigate to the project directory
cd audio-thumbnailer

# Build the project
make install
```

### Usage

The thumbnailer supports the following command line options:

```bash
audio-thumbnailer [OPTIONS] --input <input> --output <output>
```

#### Example
```bash
audio-thumbnailer -i input.mp3 -o thumbnail.png
```
#### Command line arguments
- `--input <input>`: Specifies the input audio file.
- `--output <output>`: Specifies the output file name for the generated thumbnail.

### Supported formats
| mime type | extension | waveform | cover art |
|-|-|-|-|
|`audio/mpeg`|`.m2a` `.m3a` `.mp2` `.mp2a` `.mp3` `.mpga`|✅|✅
|`audio/wav`|`.wav`|✅|❌
|`audio/ogg`|`.oga` `.ogg` `.spx`|✅|❌
|`audio/flac`|`.flac`|✅|❌
|`audio/aiff`|`.aiff` `.aif` `.aff`|✅|❌
|`audio/opus`|`.opus`|❌|❌
|`audio/x-wavpack`|`.wv`|❌|❌
|`audio/midi`|`.kar` `.mid` `.midi` `.rmi`|❌|-
|`audio/x-mpegurl`|`.m3u`|-|❌

## Contributing

Contributions are welcome! If you have any suggestions, bug fixes, or improvements, feel free to open an issue or submit a pull request.
