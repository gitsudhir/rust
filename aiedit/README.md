# Sudhir
# AIEdit - Tauri + SvelteKit + TypeScript

AIEdit is a cross-platform text editor built with Tauri, SvelteKit, and TypeScript. It provides a clean, modern interface for editing text files with native performance and CLI argument parsing capabilities.

## Features

- **Cross-platform**: Runs on Windows, macOS, and Linux
- **Lightweight**: Small bundle size thanks to Tauri's Rust backend
- **Fast**: Near-native performance with minimal resource usage
- **CLI Support**: Parse command-line arguments and subcommands
- **File Manipulation**: Read, write, and manage text files
- **Modern UI**: Clean, focused text editor interface
- **Type Safety**: Full TypeScript support throughout

## Verified Working Functionality

The application has been tested and verified to work correctly:

```bash
# Build the application
bun tauri build

# Run with a file argument
src-tauri/target/release/bundle/macos/aiedit.app/Contents/MacOS/aiedit README.md
# Output: 
# Reading file: README.md
# File read successfully
```

The application successfully reads and displays file content when provided as a CLI argument.

## Recommended IDE Setup

[VS Code](https://codevisualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Project Structure

```
├── src/                  # SvelteKit frontend
│   ├── routes/           # Application pages
│   └── app.html          # Main HTML template
├── src-tauri/            # Rust backend
│   ├── src/              # Rust source code
│   │   └── lib.rs        # Main Rust library with Tauri commands
│   ├── Cargo.toml        # Rust dependencies
│   └── tauri.conf.json   # Tauri configuration
├── static/               # Static assets
├── package.json          # Node.js dependencies
└── README.md             # This file
```

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (version 1.77.2 or higher)
- [Node.js](https://nodejs.org/) or [Bun](https://bun.sh/)
- System-specific dependencies (see [Tauri prerequisites](https://tauri.app/start/prerequisites/))

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd aiedit
```

2. Install frontend dependencies:
```bash
# Using Bun (recommended)
bun install

# Or using npm
npm install
```

3. Install Rust dependencies (automatically handled by Cargo)

### Development

Start the development server:
```bash
# Using Bun
bun run dev

# Or using npm
npm run dev
```

### Building

Build the application for production:
```bash
# Using Bun
bun run build

# Or using npm
npm run build
```

Create distributable packages:
```bash
# Build the Tauri application
bun run tauri build

# Or using npm
npm run tauri build
```

## CLI Plugin Usage

This application includes the Tauri CLI plugin for parsing command-line arguments.

### Supported Arguments

- `file`: The file to open in the editor (positional argument)
- `--input, -i <file>`: Specify an input file path
- `--output, -o <file>`: Specify an output file path
- `--verbose, -v`: Enable verbose logging
- `--theme, -t <theme>`: Set the editor theme (light/dark)

### Subcommands

- `edit <file>`: Edit a specific file
- `view <file>`: View a file (read-only mode)

### Examples

```bash
# Build the application first
bun tauri build

# Basic usage with actual executable path
src-tauri/target/release/bundle/macos/aiedit.app/Contents/MacOS/aiedit README.md
src-tauri/target/release/bundle/macos/aiedit.app/Contents/MacOS/aiedit --input document.txt --output result.txt

# With verbose logging
src-tauri/target/release/bundle/macos/aiedit.app/Contents/MacOS/aiedit -v --input document.txt

# With theme
src-tauri/target/release/bundle/macos/aiedit.app/Contents/MacOS/aiedit --theme dark README.md

# Using subcommands
src-tauri/target/release/bundle/macos/aiedit.app/Contents/MacOS/aiedit edit myfile.txt
src-tauri/target/release/bundle/macos/aiedit.app/Contents/MacOS/aiedit view myfile.txt
```

Parsed CLI arguments are displayed in the application interface for debugging purposes.

As verified in testing:
```
Reading file: README.md
File read successfully
```

## Text Editor Features

AIEdit is a focused text editor with essential file manipulation capabilities powered by Rust's standard library:

### Available Operations

- **Open File**: Load content from any text file
- **Save File**: Save content to a file (creates new or overwrites existing)
- **Create Directory**: Create new directories (including nested paths)
- **Delete File/Directory**: Remove files or entire directory trees
- **Check Existence**: Verify if a file or directory exists

### Simplified UI

The application features a clean, distraction-free interface:
- Single text editing area
- File path input field
- Essential file operation buttons (Open, Save, Check)
- Status bar for operation feedback
- CLI information and error display panels

### Rust Implementation

File operations are implemented using Rust's `std::fs` module for maximum performance and reliability:

```rust
// File reading
fs::read_to_string(path)

// File writing
fs::write(path, content)

// Directory creation
fs::create_dir_all(path)

// File/Directory deletion
fs::remove_file(path) or fs::remove_dir_all(path)

// Existence checking
Path::new(path).exists()
```

These operations are exposed to the frontend through Tauri's command system, allowing secure and efficient file manipulation from the UI.

### Viewing and Editing File Content

To view and edit file content in AIEdit:

1. **Using CLI Arguments**:
   ```bash
   # After building the application
   src-tauri/target/release/bundle/macos/aiedit.app/Contents/MacOS/aiedit README.md
   
   # Or with the --input flag
   src-tauri/target/release/bundle/macos/aiedit.app/Contents/MacOS/aiedit --input README.md
   ```

2. **Using Subcommands**:
   ```bash
   src-tauri/target/release/bundle/macos/aiedit.app/Contents/MacOS/aiedit edit README.md
   src-tauri/target/release/bundle/macos/aiedit.app/Contents/MacOS/aiedit view README.md
   ```

3. **Using the UI**:
   - Enter the file path in the input field at the top
   - Click the "Open" button to load the file content
   - Edit the content in the text area
   - Click "Save" to write changes to the file

When the application starts with a file argument, it will automatically read and display the content of the specified file. As verified in testing:

```
Reading file: README.md
File read successfully
```

## Available Scripts

- `dev`: Start the development server
- `build`: Build the frontend for production
- `preview`: Preview the production build
- `check`: Run SvelteKit type checking
- `tauri`: Run Tauri CLI commands

## Technologies Used

- [Tauri](https://tauri.app/): Build smaller, faster, and more secure desktop applications
- [SvelteKit](https://kit.svelte.dev/): The fastest way to build Svelte apps
- [TypeScript](https://www.typescriptlang.org/): Typed JavaScript at any scale
- [Vite](https://vitejs.dev/): Next generation frontend tooling
- [Rust](https://www.rust-lang.org/): Systems programming language focused on safety and performance

## Customization

### Modifying the UI

Edit files in the `src/routes/` directory to modify the frontend.

### Adding Backend Functions

1. Add new commands in `src-tauri/src/lib.rs`
2. Register them in the `invoke_handler` macro
3. Call them from the frontend using `invoke()` from `@tauri-apps/api/core`

### Extending File Operations

Add new file manipulation functions in `src-tauri/src/lib.rs` using Rust's `std::fs` module:

1. Create a new function annotated with `#[tauri::command]`
2. Implement the file operation using `std::fs`
3. Handle errors appropriately by returning `Result<T, String>`
4. Register the function in the `invoke_handler` macro

### Extending CLI Functionality

Modify the CLI argument parsing in `src/routes/+page.svelte` to handle additional arguments or subcommands.

## Troubleshooting

### Common Issues

1. **Rust compilation errors**: Ensure you have Rust 1.77.2 or higher installed
2. **Missing system dependencies**: Check Tauri's prerequisites for your OS
3. **Frontend build issues**: Clear node_modules and reinstall dependencies

### Debugging

- Check the console for JavaScript errors
- Use the Rust logs for backend debugging
- Inspect network requests in the developer tools

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Open a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Tauri](https://tauri.app/) team for the amazing framework
- [SvelteKit](https://kit.svelte.dev/) community for the excellent frontend toolkit