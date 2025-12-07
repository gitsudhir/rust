# Video Editor RS

A cross-platform desktop application built with Tauri, React, and Rust that allows users to create videos by combining images and audio files.

## Features

- Create videos from images and audio files
- Support for both local audio files and audio URLs
- Cross-platform compatibility (macOS, Windows, Linux)
- Mobile support for Android
- User-friendly interface with Material UI design

## Prerequisites

Before you begin, ensure you have the following installed:
- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/)
- [Bun](https://bun.sh/)
- [FFmpeg](https://ffmpeg.org/download.html) (for video processing)

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd video-editor-rs
   ```

2. Install dependencies:
   ```bash
   bun install
   ```

3. Install FFmpeg (if not already installed):
   - macOS: `brew install ffmpeg`
   - Ubuntu/Debian: `sudo apt install ffmpeg`
   - Windows: Download from [FFmpeg website](https://ffmpeg.org/download.html)

## Development

To run the app in development mode:

```bash
bun tauri dev
```

This will start the development server and open the application window.

## Building

To build the application for production:

```bash
bun tauri build
```

This will create distributable binaries for your platform.

## Usage

1. **Select an Image**: Click the "Select Image" button to choose an image file (PNG, JPG, JPEG).

2. **Select Audio**: Choose an audio file by clicking "Select Audio" or enter a URL to an audio file.

3. **Create Video**: Click the "Create Video" button. You'll be prompted to choose where to save the resulting video file.

4. **View and Download**: Once created, the video will be displayed in the app and can be downloaded using the "Download Video" button.

## Mobile Support

The application supports Android mobile devices:

1. Ensure you have Android Studio and the Android NDK installed
2. Connect an Android device or start an emulator
3. Run the mobile build:
   ```bash
   bun tauri android dev
   ```

## Troubleshooting

### FFmpeg Issues
If you encounter FFmpeg-related errors:
- Ensure FFmpeg is installed and accessible in your PATH
- On macOS, you may need to install via Homebrew: `brew install ffmpeg`

### Android Build Issues
If you have problems building for Android:
- Verify Android Studio and NDK are properly installed
- Ensure Java is installed and JAVA_HOME is set
- Check that the Android SDK paths are correctly configured

### Common Errors
- "Read-only file system": The app now creates videos directly where you specify, avoiding this issue
- "Failed to load image/audio": Check that the selected files are accessible and in supported formats

## Architecture

The application uses a hybrid approach for video processing:
- **Frontend**: React with TypeScript and Material UI for the user interface
- **Backend**: Rust with Tauri for system-level operations
- **Video Processing**: FFmpeg command-line tools for reliable video creation
- **File Handling**: Tauri's secure file system APIs for safe file operations

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

This project is licensed under the MIT License.