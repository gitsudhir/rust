# AIEdit - Tauri + SvelteKit + TypeScript

AIEdit is a cross-platform AI-powered text editor built with Tauri, SvelteKit, and TypeScript. It provides a clean, modern interface for editing text files with native performance, CLI argument parsing capabilities, and AI-assisted writing features.

## Features

- **Cross-platform**: Runs on Windows, macOS, and Linux
- **Lightweight**: Small bundle size thanks to Tauri's Rust backend
- **Fast**: Near-native performance with minimal resource usage
- **CLI Support**: Parse command-line arguments and subcommands
- **File Manipulation**: Read, write, and manage text files
- **AI Assistance**: Generate and append AI-powered content
- **Modern UI**: Clean, focused text editor interface
- **Type Safety**: Full TypeScript support throughout

## AI Features

AIEdit includes powerful AI-assisted writing capabilities:

### Available AI Operations

- **Generate Content**: Create new AI-generated content based on your prompts
- **Append Content**: Add AI-generated content to existing text
- **Multiple Use Cases**: Writing assistance, code generation, creative writing, etc.

### How to Use AI Features

1. Enter a prompt in the AI prompt field (e.g., "Write a short story about a robot learning to paint")
2. Click "Generate" to replace the current content with AI-generated text
3. Click "Append" to add AI-generated content to the end of existing text

### Requirements

To use AI features, you need to set your OpenAI API key as an environment variable:
```bash
export OPENAI_API_KEY="your-openai-api-key-here"
```

Then run the application as usual:
```bash
src-tauri/target/release/bundle/macos/aiedit.app/Contents/MacOS/aiedit
```

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
- [OpenAI API Key](https://platform.openai.com/api-keys) (for AI features)

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

3. Set your OpenAI API key as an environment variable:

#### For Linux/macOS:

1. Add the API key to your shell configuration file:
```bash
# For bash users, edit ~/.bashrc:
echo 'export OPENAI_API_KEY="sk-xxxxx...."' >> ~/.bashrc

# For zsh users, edit ~/.zshrc:
echo 'export OPENAI_API_KEY="sk-xxxxx...."' >> ~/.zshrc
```

2. Reload your shell configuration:
```bash
# For bash:
source ~/.bashrc

# For zsh:
source ~/.zshrc
```

3. Verify the environment variable is set:
```bash
echo $OPENAI_API_KEY
```

#### For Windows:

1. Set the environment variable temporarily:
```cmd
set OPENAI_API_KEY=sk-xxxxx....
```

2. Or set it permanently through System Properties > Environment Variables

#### Alternative Method:

You can also set the environment variable just for the current session:
```bash
export OPENAI_API_KEY="your-openai-api-key-here"
bun run tauri dev
```

4. Install Rust dependencies (automatically handled by Cargo)

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

# Simple usage (after setting up the alias as described below)
aiedit README.md
aiedit Factorial.rs
aiedit --input document.txt --output result.txt

# With verbose logging
aiedit -v --input document.txt

# With theme
aiedit --theme dark README.md

# Using subcommands
aiedit edit myfile.txt
aiedit view myfile.txt

# Original long path usage (without alias setup)
src-tauri/target/release/bundle/macos/aiedit.app/Contents/MacOS/aiedit README.md
```

### Simplified Usage Setup

To make the application easier to use, you can set up an alias:

1. Make sure you're in the project root directory
2. Run these commands to set up the alias:

```bash
# Add the alias to your shell configuration files
echo "alias aiedit='/Users/sudhirkumar/Desktop/sudhir/gitsudhir/rust/aiedit/src-tauri/target/release/bundle/macos/aiedit.app/Contents/MacOS/aiedit'" >> ~/.zshrc
echo "alias aiedit='/Users/sudhirkumar/Desktop/sudhir/gitsudhir/rust/aiedit/src-tauri/target/release/bundle/macos/aiedit.app/Contents/MacOS/aiedit'" >> ~/.bash_profile

# Source your profile to load the alias in current session
source ~/.zshrc
```

After running these commands, you can simply use:
```bash
aiedit Factorial.rs
```

Instead of the long path:
```bash
src-tauri/target/release/bundle/macos/aiedit.app/Contents/MacOS/aiedit Factorial.rs
```

Note: You only need to set up the alias once. After that, you can use the `aiedit` command from anywhere in your terminal.

Parsed CLI arguments are displayed in the application interface for debugging purposes.

As verified in testing:
```
Reading file: README.md
File read successfully
```

## Text Editor Features

AIEdit is a focused text editor with essential file manipulation capabilities and AI-assisted writing features powered by Rust's standard library and OpenAI API:

### Available Operations

- **Open File**: Load content from any text file
- **Save File**: Save content to a file (creates new or overwrites existing)
- **Create Directory**: Create new directories (including nested paths)
- **Delete File/Directory**: Remove files or entire directory trees
- **Check Existence**: Verify if a file or directory exists
- **AI Content Generation**: Generate new content using AI
- **AI Content Append**: Add AI-generated content to existing text

### Simplified UI

The application features a clean, distraction-free interface:
- Single text editing area
- File path input field
- Essential file operation buttons (Open, Save, Check)
- AI prompt input field with Generate/Append buttons
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

// AI content generation
// Uses reqwest crate to call OpenAI API
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

4. **Using AI Features**:
   - Enter a prompt in the AI prompt field (e.g., "Write a short story about a robot learning to paint")
   - Click "Generate" to replace the current content with AI-generated text
   - Click "Append" to add AI-generated content to the end of existing text

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

### Environment Variables

To use the AI features, you need to set your OpenAI API key as an environment variable. Here's how to do it properly:

#### For Linux/macOS:

1. Add the API key to your shell configuration file:
```bash
# For bash users, edit ~/.bashrc:
echo 'export OPENAI_API_KEY="sk-xxxxx...."' >> ~/.bashrc

# For zsh users, edit ~/.zshrc:
echo 'export OPENAI_API_KEY="sk-xxxxx...."' >> ~/.zshrc
```

2. Reload your shell configuration:
```bash
# For bash:
source ~/.bashrc

# For zsh:
source ~/.zshrc
```

3. Verify the environment variable is set:
```bash
echo $OPENAI_API_KEY
```

#### For Windows:

1. Set the environment variable temporarily:
```cmd
set OPENAI_API_KEY=sk-xxxxx....
```

2. Or set it permanently through System Properties > Environment Variables

#### Alternative Method:

You can also set the environment variable just for the current session:
```bash
export OPENAI_API_KEY="your-openai-api-key-here"
bun run tauri dev
```

**Note**: Never commit your API key to version control. The `.gitignore` file is already configured to exclude environment files.

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

### Extending AI Features

Add new AI features in `src-tauri/src/lib.rs`:

1. Create a new function that uses the `reqwest` crate to call external APIs
2. Handle API keys securely through environment variables
3. Register the function in the `invoke_handler` macro
4. Call the function from the frontend using `invoke()` from `@tauri-apps/api/core`

### Extending CLI Functionality

Modify the CLI argument parsing in `src/routes/+page.svelte` to handle additional arguments or subcommands.

### Troubleshooting AI Features

If you encounter issues with AI features, here are common solutions:

1. **"AI generation failed: Failed to extract generated text from response"**
   - Check that your OpenAI API key is valid and has sufficient credits
   - Verify the API key is properly set in environment variables
   - Check your internet connection
   - Look at the console logs for detailed error information

2. **"OPENAI_API_KEY environment variable not set"**
   - Ensure you've properly set the environment variable
   - On Linux/macOS, reload your shell with `source ~/.bashrc` or `source ~/.zshrc`
   - On Windows, restart your command prompt or IDE

3. **Rate Limiting**
   - If you see rate limit errors, wait a few minutes before trying again
   - Consider upgrading your OpenAI plan for higher rate limits

4. **Network Issues**
   - Ensure you have internet connectivity
   - Check if your firewall or proxy is blocking the request

5. **Debugging**
   - Open the developer console (Ctrl+Shift+I or Cmd+Option+I) to see detailed error logs
   - The Rust backend prints debug information to the console when processing AI requests

## Troubleshooting

### Common Issues

1. **Rust compilation errors**: Ensure you have Rust 1.77.2 or higher installed
2. **Missing system dependencies**: Check Tauri's prerequisites for your OS
3. **Frontend build issues**: Clear node_modules and reinstall dependencies

### Debugging

- Check the console for JavaScript errors
- Use the Rust logs for backend debugging
- Inspect network requests in the developer tools

See the specific AI Features Troubleshooting section for AI-related issues.

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