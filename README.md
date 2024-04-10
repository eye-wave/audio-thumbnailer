# Audio File Thumbnailer

This is a fast and lightweight audio file thumbnailer written in Rust. It generates thumbnails for audio files that can be used by file managers.

## Features

- **Fast and Lightweight**: Thanks to rust programming language.
- **Customizable**: You can specify percisely how you want your thumbnail to look, by customizing ~~quality~~ , aspect ratio treatment, interpolation algorithm, waveform color, and waveform size.
- **Multipurpose**: It can generate album covers and for audio files without embedded cover art it generates waveforms to represent the audio content visually. 


## Usage

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

### Command Line Options

The thumbnailer supports the following command line options:

```bash
audio-thumbnailer [OPTIONS] --input <input> --output <output>
```

- `--input <input>`: Specifies the input audio file.
- `--output <output>`: Specifies the output file name for the generated thumbnail.
~~- `--quality <quality>`: Specifies the quality of the generated image (0 = worst, 10 = lossless).~~
- `--aspect_ratio <aspect_ratio>`: Specifies how to treat aspect ratio ("crop", "stretch", "auto").
- `--size <size>`: Specifies the thumbnail size.
- `--color <color>`: Specifies the waveform color.
- `--interpol <interpol>`: Specifies the interpolation algorithm to use ("lanczos3", "gaussian", "nearest", "triangle").
- `--waveform_size <waveform_size>`: Specifies the waveform width and height.

### Example

```bash
audio-thumbnailer -i input.mp3 -o thumbnail.png
```

## Contributing

Contributions are welcome! If you have any suggestions, bug fixes, or improvements, feel free to open an issue or submit a pull request.
