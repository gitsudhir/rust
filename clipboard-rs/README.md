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

## macOS Installation Issues

If you encounter the error "'clipboard-rs' is damaged and can't be opened. You should move it to the Bin," this is due to macOS Gatekeeper security features. Here are several solutions:

### Solution 1: Bypass Gatekeeper (Quick Fix)
Try running this command in Terminal:
```bash
xattr -d com.apple.quarantine /Applications/clipboard-rs.app
```

Or if the app is in your Downloads folder:
```bash
xattr -d com.apple.quarantine ~/Downloads/clipboard-rs.app
```

### Solution 2: Right-click Open Method
1. Navigate to the app in Finder
2. Right-click (or Control-click) on the app
3. Select "Open" from the context menu
4. Click "Open" in the dialog that appears
5. The app should now open normally

### Solution 3: Codesign Command
If the above methods don't work, try signing the app:
```bash
codesign --force --deep --sign - /Applications/clipboard-rs.app
```

### Solution 4: Developer Signing (Production)
For a permanent solution, join the Apple Developer Program and obtain a Developer ID certificate. Update the Tauri configuration with your signing identity:
```json
"macOS": {
  "signingIdentity": "Developer ID Application: Your Name (TEAMID)",
  "providerShortName": "TEAMID"
}
```

### Solution 5: Disable Gatekeeper Temporarily (Not Recommended for Regular Use)
```bash
# Disable (use only for testing)
sudo spctl --master-disable

# Re-enable after testing
sudo spctl --master-enable
```

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