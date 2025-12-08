# Learn Tauri: Building Desktop Applications with Rust and React

## Table of Contents
1. [Introduction to Tauri](#introduction-to-tauri)
2. [Architecture Overview](#architecture-overview)
3. [Project Structure](#project-structure)
4. [Backend Development with Rust](#backend-development-with-rust)
5. [Frontend Development with React](#frontend-development-with-react)
6. [Communication Between Frontend and Backend](#communication-between-frontend-and-backend)
7. [Core Tauri Modules and Functions](#core-tauri-modules-and-functions)
8. [Plugins and Extensions](#plugins-and-extensions)
9. [Building and Deployment](#building-and-deployment)
10. [Best Practices](#best-practices)

## Introduction to Tauri

Tauri is a framework for building tiny, blazing fast binaries for all major desktop platforms. Developers can integrate any front-end framework that compiles to HTML, JS and CSS for building their user interface. The backend of the application is a Rust-powered secure runtime with an API that is mockable during development and testable without a browser.

### Key Features
- **Tiny Bundle Size**: Tauri apps are typically much smaller than Electron apps
- **Performance**: Leverages Rust's performance and memory safety
- **Security**: Sandboxed runtime with fine-grained API control
- **Cross-Platform**: Build for Windows, macOS, and Linux from a single codebase
- **Flexible Frontend**: Use any frontend framework (React, Vue, Svelte, etc.)

## Architecture Overview

Tauri follows a hybrid architecture where:
- **Frontend**: Runs in a lightweight webview (WebView2 on Windows, WKWebView on macOS, WebViewGTK/Webkit2GTK on Linux)
- **Backend**: Rust code that handles system interactions, business logic, and security-sensitive operations
- **Communication**: Inter-process communication (IPC) between frontend and backend using JSON messages

```
┌─────────────────┐    IPC    ┌──────────────────┐
│   Frontend      │ ◄───────► │    Backend       │
│  (React/Vue/etc)│    JSON   │     (Rust)       │
└─────────────────┘           └──────────────────┘
         │                              │
         ▼                              ▼
  ┌─────────────┐              ┌─────────────────┐
  │   WebView   │              │ System APIs     │
  │   Engine    │              │ (Filesystem,    │
  └─────────────┘              │  Network, etc.) │
                               └─────────────────┘
```

## Project Structure

A typical Tauri project has the following structure:

```
my-tauri-app/
├── src/                    # Frontend source code (React)
│   ├── App.tsx
│   ├── main.tsx
│   └── ...
├── src-tauri/              # Backend source code (Rust)
│   ├── src/
│   │   └── main.rs         # Main Rust entry point
│   ├── Cargo.toml          # Rust dependencies
│   ├── tauri.conf.json     # Tauri configuration
│   └── build.rs            # Build script
├── package.json            # Frontend dependencies
├── index.html              # Entry HTML file
└── ...
```

## Backend Development with Rust

### Core Components

#### 1. Main Entry Point (`src-tauri/src/main.rs`)

```rust
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

#### 2. Commands
Commands are functions that can be called from the frontend:

```rust
#[tauri::command]
fn my_command(argument: &str) -> Result<String, String> {
    // Business logic here
    Ok(format!("Processed: {}", argument))
}
```

#### 3. Setup Hook
Used for initialization tasks when the app starts:

```rust
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialization code here
            println!("App is starting up!");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Important Rust Modules

#### Tauri Core Modules
- `tauri::command`: Macro for creating callable commands
- `tauri::Builder`: Builder pattern for configuring the app
- `tauri::AppHandle`: Handle to the running application
- `tauri::Window`: Reference to application windows
- `tauri::Manager`: Trait for managing windows and other resources

#### Common Patterns

##### Error Handling
```rust
#[tauri::command]
fn fallible_command() -> Result<String, String> {
    // This will return an error to the frontend
    Err("Something went wrong!".to_string())
}
```

##### Async Commands
```rust
#[tauri::command]
async fn async_command() -> String {
    // Perform async operations
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    "Done!".to_string()
}
```

##### State Management
```rust
struct MyState {
    value: Mutex<String>,
}

#[tauri::command]
fn update_state(state: tauri::State<MyState>, new_value: &str) {
    *state.value.lock().unwrap() = new_value.to_string();
}
```

## Frontend Development with React

### Core Components

#### 1. Invoking Commands
Using the `@tauri-apps/api` package:

```javascript
import { invoke } from '@tauri-apps/api/tauri';

function MyComponent() {
    const handleClick = async () => {
        try {
            const result = await invoke('greet', { name: 'World' });
            console.log(result);
        } catch (error) {
            console.error(error);
        }
    };

    return <button onClick={handleClick}>Greet</button>;
}
```

#### 2. Event Handling
Listening for events from the backend:

```javascript
import { listen } from '@tauri-apps/api/event';

function MyComponent() {
    useEffect(() => {
        const unlisten = listen('event-name', (event) => {
            console.log('Received event:', event.payload);
        });

        return () => {
            unlisten.then(f => f());
        };
    }, []);
}
```

#### 3. Window Management
Controlling the application window:

```javascript
import { appWindow } from '@tauri-apps/api/window';

async function closeWindow() {
    await appWindow.close();
}
```

## Communication Between Frontend and Backend

### 1. Commands (Frontend → Backend)
Commands are the primary way to call Rust functions from JavaScript:

```rust
// Backend (Rust)
#[tauri::command]
fn process_data(input: &str) -> Result<String, String> {
    // Process the data
    Ok(format!("Processed: {}", input))
}
```

```javascript
// Frontend (JavaScript)
const result = await invoke('process_data', { input: 'Hello' });
```

### 2. Events (Bidirectional)
Events allow for asynchronous communication:

```rust
// Backend (Rust)
use tauri::Emitter;

fn emit_event(app_handle: &tauri::AppHandle) {
    app_handle.emit("data-update", "New data").unwrap();
}
```

```javascript
// Frontend (JavaScript)
import { listen } from '@tauri-apps/api/event';

listen('data-update', (event) => {
    console.log('Received:', event.payload);
});
```

### 3. State Management
Sharing state between commands:

```rust
// Backend (Rust)
struct AppState {
    counter: Mutex<i32>,
}

#[tauri::command]
fn increment_counter(state: tauri::State<AppState>) -> i32 {
    let mut counter = state.counter.lock().unwrap();
    *counter += 1;
    *counter
}
```

## Core Tauri Modules and Functions

### 1. Command Module
The `#[tauri::command]` macro is essential for creating functions callable from the frontend.

#### Attributes
- `rename_all`: Control parameter naming (snake_case vs camelCase)
- `async`: Mark async functions

```rust
#[tauri::command(rename_all = "snake_case")]
fn my_command(param_name: &str) -> String {
    param_name.to_string()
}
```

### 2. Builder Module
Configures and builds the Tauri application:

```rust
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![cmd1, cmd2])
    .setup(|app| { /* setup code */ })
    .manage(MyState::default())
    .run(tauri::generate_context!())
```

### 3. AppHandle Module
Provides access to the running application:

```rust
#[tauri::command]
fn app_info(app: tauri::AppHandle) -> String {
    format!("App version: {:?}", app.package_info().version)
}
```

### 4. Window Module
Controls application windows:

```rust
#[tauri::command]
async fn resize_window(window: tauri::Window) {
    window.set_size(tauri::PhysicalSize::new(800, 600)).unwrap();
}
```

### 5. Manager Module
Manages application resources:

```rust
#[tauri::command]
fn create_new_window(app: tauri::AppHandle) -> tauri::Result<()> {
    app.get_window("main").unwrap().emit("new-window", ())?;
    Ok(())
}
```

### 6. Dialog Module
Shows native dialogs:

```rust
use tauri::api::dialog::FileDialogBuilder;

#[tauri::command]
async fn open_file_dialog(app: tauri::AppHandle) -> Option<String> {
    FileDialogBuilder::new()
        .pick_file()
        .and_then(|path| path.to_str().map(|s| s.to_string()))
}
```

## Plugins and Extensions

### Official Plugins

#### Clipboard Manager
```rust
// Cargo.toml
tauri-plugin-clipboard-manager = "0.1"

// main.rs
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .run(tauri::generate_context!())
}
```

```rust
#[tauri::command]
fn read_clipboard_text(app: tauri::AppHandle) -> Result<String, String> {
    app.clipboard().read_text().map_err(|e| e.to_string())
}
```

#### Store Plugin
```rust
// Cargo.toml
tauri-plugin-store = "0.1"

// main.rs
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
}
```

```rust
#[tauri::command]
fn save_data(app: tauri::AppHandle, key: &str, value: &str) -> Result<(), String> {
    let store = app.store("data.bin").map_err(|e| e.to_string())?;
    store.set(key, value.into()).map_err(|e| e.to_string())?;
    store.save().map_err(|e| e.to_string())?;
    Ok(())
}
```

### Custom Plugins
Creating custom plugins:

```rust
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("my-plugin")
        .invoke_handler(tauri::generate_handler![my_command])
        .build()
}
```

## Building and Deployment

### Development Workflow
```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### Configuration (`tauri.conf.json`)
```json
{
  "build": {
    "distDir": "../dist",
    "devPath": "http://localhost:5173",
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build"
  },
  "tauri": {
    "bundle": {
      "active": true,
      "targets": "all",
      "identifier": "com.myapp.app",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ]
    }
  }
}
```

### Platform-Specific Builds
```bash
# Build for specific platforms
npm run tauri build -- --target x86_64-pc-windows-msvc
npm run tauri build -- --target x86_64-apple-darwin
npm run tauri build -- --target x86_64-unknown-linux-gnu
```

## Best Practices

### 1. Security
- Validate all inputs from the frontend
- Use the principle of least privilege
- Sanitize file paths to prevent directory traversal

```rust
#[tauri::command]
fn safe_file_operation(path: &str) -> Result<String, String> {
    // Validate path
    if path.contains("..") {
        return Err("Invalid path".to_string());
    }
    
    // Proceed with safe operations
    Ok("Success".to_string())
}
```

### 2. Error Handling
- Always return `Result` types from commands
- Provide meaningful error messages
- Log errors appropriately

```rust
#[tauri::command]
fn robust_command(input: &str) -> Result<String, String> {
    if input.is_empty() {
        return Err("Input cannot be empty".to_string());
    }
    
    // Process input
    Ok(format!("Processed: {}", input))
}
```

### 3. Performance
- Use async operations for I/O bound tasks
- Avoid blocking the main thread
- Cache expensive computations

```rust
#[tauri::command]
async fn async_operation() -> String {
    // Simulate async work
    tokio::task::spawn_blocking(|| {
        // Heavy computation here
        "Result".to_string()
    }).await.unwrap()
}
```

### 4. State Management
- Use `tauri::State` for shared state
- Protect shared data with mutexes
- Initialize state during app setup

```rust
struct AppState {
    data: Mutex<HashMap<String, String>>,
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            data: Mutex::new(HashMap::new()),
        })
        .run(tauri::generate_context!())
}
```

### 5. Testing
- Write unit tests for business logic
- Test commands with mock data
- Use integration tests for complex workflows

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_my_function() {
        assert_eq!(my_function("input"), "expected_output");
    }
}
```

## Conclusion

Tauri provides a powerful framework for building desktop applications with the performance of Rust and the flexibility of web technologies. By understanding the core concepts of commands, events, and the communication patterns between frontend and backend, developers can create robust, secure, and performant desktop applications.

The key advantages of Tauri include:
- Small bundle sizes compared to Electron
- Memory safety through Rust
- Native performance
- Cross-platform compatibility
- Rich ecosystem of plugins

With this guide, you should have a solid foundation for building your own Tauri applications and understanding how the various components work together to create a seamless desktop experience.