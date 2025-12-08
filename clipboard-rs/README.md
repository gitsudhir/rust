# Clipboard Manager - Tauri + React + Typescript

This is a clipboard manager application built with Tauri, React, and Typescript in Vite. It allows users to copy text to the system clipboard, paste text from the system clipboard, and maintain a history of copied items with automatic monitoring, including full image support. The UI is built with Material UI and includes animations for a better user experience.

## Features

- Copy text to clipboard
- Paste text from clipboard
- Automatic clipboard monitoring (captures Command+C/Ctrl+C)
- Clipboard history with persistent storage (text and images)
- Image preview in history with dimensions
- Copy images from history back to clipboard
- Clear clipboard history
- Modern Material UI design with animations
- Responsive design for different screen sizes
- Cross-platform support (Windows, macOS, Linux)

## Documentation

For detailed information about the application architecture and Tauri development:

- [Learn Tauri Guide](./learn-tauri.md) - Comprehensive guide to Tauri framework
- [Application Architecture](./clipboard-rs-architecture.md) - Detailed documentation of this application's implementation

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Getting Started

1. Install dependencies:
   ```bash
   bun install
   ```

2. Run the development server:
   ```bash
   bun run tauri dev
   ```

3. Build the application:
   ```bash
   bun run tauri build
   ```

## Usage

- Enter text in the input field and click "Copy to Clipboard" to copy text to the system clipboard
- Click "Paste from Clipboard" to read text from the system clipboard and display it
- Use Command+C (macOS) or Ctrl+C (Windows/Linux) anywhere in the system to automatically capture copied text to history
- Images copied to clipboard will also be detected, saved to temporary files, and shown with dimensions in history (e.g., "[Image] 1920x1080")
- View clipboard history in the history section with visual indicators for text and images
- Click "Copy" button next to any history item to copy it back to the clipboard (works for both text and images)
- Click "Refresh History" to reload the clipboard history
- Click "Clear History" to remove all saved clipboard items

## Project Structure

- `src/` - Frontend React components and logic
- `src/theme.ts` - Material UI theme customization
- `src-tauri/` - Backend Rust code and Tauri configuration
- `src-tauri/src/lib.rs` - Main Rust backend with clipboard commands, history functionality, and monitoring
- `src/App.tsx` - Main React component with clipboard UI and history display
- `learn-tauri.md` - Comprehensive Tauri learning guide
- `clipboard-rs-architecture.md` - Detailed application architecture documentation

## Dependencies

- Tauri v2
- React v19
- TypeScript
- Material UI (@mui/material)
- Framer Motion (for animations)
- tauri-plugin-clipboard-manager
- tauri-plugin-store
- base64 (for image encoding)
- uuid (for generating unique filenames)
- image (for image processing)