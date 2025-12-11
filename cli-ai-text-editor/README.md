# Tauri + Angular CLI Text Editor

This is a desktop text editor application built with Tauri and Angular that includes advanced CLI (Command Line Interface) functionality. The application can be controlled via command-line arguments when launched from the terminal and can display file contents.

## Features

- Desktop text editor with Angular frontend
- Rust backend powered by Tauri
- Advanced command-line interface support with configurable arguments
- File opening and content display through CLI
- Theme selection (light/dark mode)
- Verbose logging option
- Output file specification
- Subcommands for different operations

## Recommended IDE Setup

[VS Code](https://codevisualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) + [Angular Language Service](https://marketplace.visualstudio.com/items?itemName=Angular.ng-template).

## CLI Configuration

The application supports the following command-line arguments:

### Arguments

1. **File Argument** (Positional)
   - Opens and displays the content of a specified file when the application starts
   - Usage: `./cli-ai-text-editor myfile.txt`

2. **Output Option** (`-o`)
   - Specifies an output file path for saving operations
   - Usage: `./cli-ai-text-editor myfile.txt -o output.txt`

3. **Verbose Flag** (`-v`)
   - Enables verbose logging for debugging purposes
   - Usage: `./cli-ai-text-editor -v`

4. **Theme Option** (`-t`)
   - Sets the editor theme (light or dark)
   - Usage: `./cli-ai-text-editor -t dark`

### Subcommands

1. **edit** - Edit a file
   - Usage: `./cli-ai-text-editor edit myfile.txt`

2. **view** - View a file (read-only mode)
   - Usage: `./cli-ai-text-editor view myfile.txt`

### Example Commands

```bash
# Open and display a file
./cli-ai-text-editor document.txt

# Open a file with verbose logging
./cli-ai-text-editor document.txt -v

# Open and display a file with dark theme
./cli-ai-text-editor document.txt -t dark

# Open a file and specify output location
./cli-ai-text-editor input.txt -o output.txt

# Use multiple options together
./cli-ai-text-editor document.txt -o result.txt -v -t light

# Use subcommands
./cli-ai-text-editor edit myfile.txt
./cli-ai-text-editor view myfile.txt
```

Note: When running the application with CLI arguments, a desktop window will open displaying the parsed arguments and file content (if a file was specified). The CLI functionality is primarily designed to pass arguments to the application, which then processes them in the GUI.

## Project Structure

- `src/` - Angular frontend application
- `src-tauri/` - Tauri backend and desktop integration
- `src-tauri/src/` - Rust source code
- `src-tauri/tauri.conf.json` - Tauri configuration including CLI setup
- `src/app/services/cli.service.ts` - Angular service for handling CLI arguments and file operations

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/)
- [Node.js](https://nodejs.org/) or [Bun](https://bun.sh/)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/)

### Installation

```bash
# Install frontend dependencies
bun install

# Install Tauri CLI globally (if not already installed)
cargo install tauri-cli
```

### Running the Application

```bash
# Development mode
bun run tauri dev

# Build for production
bun run tauri build
```

## CLI Implementation Details

The CLI functionality is implemented using the Tauri CLI plugin:

1. **Configuration**: Defined in `src-tauri/tauri.conf.json` under the "plugins" section
2. **Permissions**: Enabled in `src-tauri/capabilities/default.json`
3. **Backend Integration**: Rust plugin initialization in `src-tauri/src/lib.rs`
4. **Frontend Integration**: Angular service in `src/app/services/cli.service.ts`

### Extending CLI Functionality

To add new command-line arguments:

1. Modify the `plugins.cli` section in `src-tauri/tauri.conf.json`
2. Update the `CliService` in `src/app/services/cli.service.ts` to handle new arguments
3. Use the parsed arguments in your application components

## Building the Application

```bash
# Build the Angular frontend
bun run build

# Build the complete Tauri application
bun run tauri build
```

The built application will be available in the `src-tauri/target/release/bundle/` directory.

On macOS, the application bundle can be found at:
`src-tauri/target/release/bundle/macos/cli-ai-text-editor.app`

To run the application with CLI arguments:
`src-tauri/target/release/bundle/macos/cli-ai-text-editor.app/Contents/MacOS/cli-ai-text-editor [arguments]`

## Using the CLI

After building the application, you can use the CLI functionality by running the executable with arguments:

1. **Basic usage**:
   ```bash
   ./cli-ai-text-editor
   ```
   This will launch the application without any specific arguments.

2. **Open and display a file**:
   ```bash
   ./cli-ai-text-editor myfile.txt
   ```
   This will launch the application, read the content of "myfile.txt", and display it in the UI.

3. **Use flags**:
   ```bash
   ./cli-ai-text-editor -v
   ```
   This will launch the application with the verbose flag enabled.

4. **Set theme**:
   ```bash
   ./cli-ai-text-editor -t dark
   ```
   This will launch the application with the dark theme.

5. **Combine arguments**:
   ```bash
   ./cli-ai-text-editor myfile.txt -o output.txt -v -t light
   ```
   This will launch the application with a file argument, output option, verbose flag, and light theme.

6. **Use subcommands**:
   ```bash
   ./cli-ai-text-editor edit myfile.txt
   ./cli-ai-text-editor view myfile.txt
   ```

### What to Expect

When you run the application with CLI arguments:

1. A desktop window will appear (since this is a Tauri desktop application)
2. The parsed CLI arguments will be displayed in the application's main window
3. If a file was specified, its content will be displayed in the UI
4. If a theme is specified, the application UI will adjust accordingly
5. The application will not exit immediately but will remain open as a desktop application
6. No output will be printed to the terminal (as this is a GUI application)

### Verification

You can verify that the CLI plugin is working correctly by running the application with cargo directly:

```bash
cd src-tauri
cargo run -- test.txt
```

This will show output like:
```
CLI Matches: Matches { args: {"theme": ArgData { value: Null, occurrences: 0 }, "file": ArgData { value: String("test.txt"), occurrences: 1 }, "output": ArgData { value: Null, occurrences: 0 }, "verbose": ArgData { value: Bool(false), occurrences: 0 }}, subcommand: None }
Reading file: test.txt
File read successfully
```

This confirms that:
1. The CLI arguments are being parsed correctly by the Tauri CLI plugin
2. The file content is being read successfully by the Rust backend

### Troubleshooting

If the application doesn't seem to be working correctly:

1. **Ensure the application is built**:
   ```bash
   bun run tauri build
   ```

2. **Use the full path to the executable**:
   ```bash
   src-tauri/target/release/bundle/macos/cli-ai-text-editor.app/Contents/MacOS/cli-ai-text-editor test.txt
   ```

3. **Create a symlink for easier access**:
   ```bash
   ln -s src-tauri/target/release/bundle/macos/cli-ai-text-editor.app/Contents/MacOS/cli-ai-text-editor ./cli-ai-text-editor
   ./cli-ai-text-editor test.txt
   ```

4. **Check that the file exists and is readable**:
   ```bash
   ls -la test.txt
   cat test.txt
   ```

5. **Run in development mode to see console output**:
   ```bash
   bun run tauri dev
   ```

The CLI functionality is designed to allow users to pass configuration options to the application at startup, similar to how many desktop applications accept command-line arguments. The parsed arguments are then available within the application for processing.

The application demonstrates successful integration of the Tauri CLI plugin, showing that command-line arguments can be passed to and processed by a Tauri desktop application, including file reading capabilities.