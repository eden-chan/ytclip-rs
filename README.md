# ytclip-rs

A fast and efficient YouTube video clip downloader written in Rust. Extract specific time segments without downloading entire videos.

## Features

- **Stream-based clipping** - Downloads only the specified segment, not the entire video
- **Timestamp support** - Specify exact start and end times
- **Speed adjustment** - Change playback speed (0.5x to 4x)
- **QuickTime compatible** - Outputs H.264/AAC MP4 files
- **Minimal dependencies** - Just needs yt-dlp and ffmpeg

## Installation

### Prerequisites

Install required tools:
```bash
# macOS
brew install yt-dlp ffmpeg

# Ubuntu/Debian
sudo apt update
sudo apt install yt-dlp ffmpeg

# Windows (using Chocolatey)
choco install yt-dlp ffmpeg
```

### Build from source

```bash
# Clone the repository
git clone https://github.com/eden-chan/ytclip-rs.git
cd ytclip-rs

# Build with cargo
cargo build --release

# Install globally (optional)
cargo install --path .
```

## Usage

### Basic usage

```bash
ytclip "https://www.youtube.com/watch?v=VIDEO_ID" START_TIME END_TIME
```

### Examples

```bash
# Clip from 1:30 to 2:45
ytclip "https://www.youtube.com/watch?v=dQw4w9WgXcQ" 1:30 2:45

# Clip with custom output name
ytclip "https://www.youtube.com/watch?v=dQw4w9WgXcQ" 0:30 1:00 -o my_clip.mp4

# Clip at 1.5x speed
ytclip "https://www.youtube.com/watch?v=dQw4w9WgXcQ" 0:00 0:30 -s 1.5

# Using seconds notation
ytclip "https://www.youtube.com/watch?v=dQw4w9WgXcQ" 90 150

# Using hours:minutes:seconds notation
ytclip "https://www.youtube.com/watch?v=dQw4w9WgXcQ" 1:30:00 1:31:30
```

### Command-line options

```
ytclip [OPTIONS] <URL> <START_TIME> <END_TIME>

Arguments:
  <URL>         YouTube URL to download from
  <START_TIME>  Start time (e.g., 1:30, 90, 1:30:45)
  <END_TIME>    End time (e.g., 2:45, 165, 2:45:30)

Options:
  -o, --output <OUTPUT>  Custom output filename (optional)
  -s, --speed <SPEED>    Playback speed (0.5 to 4.0) [default: 1.0]
  -h, --help            Print help
  -V, --version         Print version
```

## Time Format

Supported time formats:
- **Seconds**: `90` (90 seconds)
- **Minutes:Seconds**: `1:30` (1 minute 30 seconds)
- **Hours:Minutes:Seconds**: `1:30:45` (1 hour 30 minutes 45 seconds)

## Performance

The Rust implementation offers:
- Fast startup time
- Low memory usage
- Efficient stream processing
- Parallel processing capabilities
- Zero-cost abstractions

## Building

### Release build (optimized)
```bash
cargo build --release
```

### Development build
```bash
cargo build
cargo run -- "URL" "START" "END"
```

### Run tests
```bash
cargo test
```

## License

MIT License - see LICENSE file for details

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Comparison with Python version

This Rust implementation offers several advantages over the Python version:
- **Faster execution** - Compiled binary with no interpreter overhead
- **Lower memory usage** - Efficient memory management
- **Better error handling** - Rust's Result type for robust error handling
- **Type safety** - Compile-time type checking prevents runtime errors
- **Single binary** - No Python or pip required for distribution