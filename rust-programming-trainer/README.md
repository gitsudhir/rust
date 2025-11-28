# RUST System Programming

## Overall Training Objective
By the end of this program, participants will:
- Understand Rust's core principles (ownership, borrowing, lifetimes) and why it is ideal for system-level programming.
- Gain proficiency in writing safe, efficient, and concurrent Rust code.
- Learn to develop system-level applications for Linux and integrate Rust into Android environments.
- Apply Rust for OS-level tasks such as process management, IPC, networking, and FFI with C/C++.
- Build and deploy real-world system utilities and Android-native components using Rust.

## Introduction to Rust & Systems Foundations

### Learning Outcomes:
- Understand Rust's role in system programming
- Set up development environment
- Compile and run basic programs
- Grasp fundamental systems programming concepts

### Topics:

#### Why Rust for Systems Programming
Rust is a systems programming language that prioritizes safety, speed, and concurrency. Unlike traditional systems languages like C and C++, Rust prevents segmentation faults and buffer overflows at compile time without requiring a garbage collector. This makes it ideal for:
- Operating system development
- Device drivers
- Embedded systems
- High-performance web services
- Blockchain implementations
- Game engines

#### Installing Rust (rustup)
Rust is installed using `rustup`, the official Rust version manager:
```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add Rust to PATH
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

#### Cargo Basics
Cargo is Rust's package manager and build system:
```bash
# Create a new project
cargo new hello_world

# Build project
cargo build

# Build for release
cargo build --release

# Run project
cargo run

# Run tests
cargo test

# Generate documentation
cargo doc
```

#### IDE Setup
Recommended IDEs for Rust development:
1. **Visual Studio Code** with rust-analyzer extension
2. **IntelliJ IDEA** with Rust plugin
3. **vim/neovim** with rust.vim plugin

#### Writing First Rust Program
```rust
// main.rs
fn main() {
    println!("Hello, systems world!");
}
```

#### What is Systems Programming?
Systems programming involves creating software that interacts closely with computer hardware and operating systems. This includes:
- Operating systems
- Device drivers
- Firmware
- Embedded systems
- Real-time systems
- Network infrastructure

#### OS, Hardware & CPU Basics
Key concepts:
- **CPU Architecture**: Registers, ALU, Control Unit
- **Memory Hierarchy**: Registers → Cache → RAM → Storage
- **Processes**: Execution units managed by the OS
- **Kernel Space vs User Space**: Privilege levels in OS
- **System Calls**: Interface between user programs and kernel

#### Rust Toolchain & Compiler Pipeline
The Rust compilation process:
1. **Parsing**: Source code → Abstract Syntax Tree (AST)
2. **Name Resolution**: Resolving identifiers
3. **Type Checking**: Ensuring type safety
4. **Borrow Checking**: Enforcing ownership rules
5. **Code Generation**: LLVM IR generation
6. **Optimization**: LLVM optimizations
7. **Machine Code**: Target-specific assembly

#### Variables, Functions, Control Flow
```rust
// Variables
let immutable_var = 5;
let mut mutable_var = 10;
const CONSTANT_VAR: i32 = 100;

// Functions
fn add(a: i32, b: i32) -> i32 {
    a + b // Implicit return
}

// Control Flow
if condition {
    // Do something
} else if another_condition {
    // Do something else
} else {
    // Default case
}

// Loops
for i in 0..10 {
    println!("{}", i);
}

while condition {
    // Loop body
}
```

#### Expressions vs Statements
- **Statements**: Perform actions, don't return values (e.g., `let x = 5;`)
- **Expressions**: Evaluate to a value (e.g., `5 + 3`, `{ let x = 5; x + 1 }`)

#### Hands-on Project: Hex Dump Tool
Create a tool that displays file contents in hexadecimal format:
```rust
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }
    
    let filename = &args[1];
    let mut file = File::open(filename).expect("Failed to open file");
    let mut buffer = [0; 16];
    
    loop {
        let bytes_read = file.read(&mut buffer).expect("Failed to read file");
        if bytes_read == 0 {
            break;
        }
        
        // Print hex values
        for byte in &buffer[..bytes_read] {
            print!("{:02x} ", byte);
        }
        println!();
    }
}
```

## Rust Fundamentals

### Learning Outcomes:
- Able to understand Rust syntax and unique concepts like ownership and borrowing
- Able to write safe, modular code
- Understand error handling patterns in Rust

### Topics:

#### Variables, Data Types, Control Flow
Rust has a strong static type system with type inference:
```rust
// Scalar Types
let integer: i32 = 42;          // Signed 32-bit integer
let float: f64 = 3.14;          // 64-bit floating point
let boolean: bool = true;       // Boolean
let character: char = 'R';      // Unicode scalar value

// Compound Types
let tuple: (i32, f64, char) = (42, 3.14, 'R');
let array: [i32; 3] = [1, 2, 3]; // Fixed-size array
let slice: &[i32] = &array[1..3]; // Dynamic view into data

// Shadowing
let x = 5;
let x = x + 1; // New variable x shadows previous x
```

#### Functions & Modules
Functions in Rust:
```rust
// Basic function
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// Function with multiple parameters
fn calculate_area(length: f64, width: f64) -> f64 {
    length * width
}

// Module system
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    fn private_helper() {
        // Not accessible outside module
    }
}

use math::add; // Import specific function
```

#### Ownership, Borrowing, Lifetimes
Core concepts that make Rust memory safe:

##### Ownership Rules:
1. Each value has a variable that's called its owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value is dropped

```rust
fn main() {
    let s1 = String::from("hello"); // s1 owns the string
    let s2 = s1;                    // s1 moves to s2, s1 is no longer valid
    
    // println!("{}", s1);          // This would cause a compile error
    println!("{}", s2);             // This works
}
```

##### Borrowing:
```rust
fn main() {
    let s = String::from("hello");
    
    // Immutable borrow
    let len = calculate_length(&s);
    println!("Length: {}", len);
    
    // Mutable borrow
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn calculate_length(s: &String) -> usize {
    s.len() // s is borrowed, not owned
}

fn change(s: &mut String) {
    s.push_str(", world!"); // Modify through mutable reference
}
```

##### Lifetimes:
```rust
// Explicit lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

#### Error Handling (Result, Option)
Rust uses algebraic data types for error handling:

##### Option<T> for nullable values:
```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b != 0.0 {
        Some(a / b)
    } else {
        None
    }
}

// Using Option
match divide(10.0, 2.0) {
    Some(result) => println!("Result: {}", result),
    None => println!("Cannot divide by zero"),
}
```

##### Result<T, E> for recoverable errors:
```rust
use std::fs::File;
use std::io::Read;

fn read_file_contents(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Using Result
match read_file_contents("data.txt") {
    Ok(contents) => println!("File contents: {}", contents),
    Err(error) => println!("Error reading file: {}", error),
}
```

#### Pattern Matching
Powerful destructuring capabilities:
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
        Message::Write(text) => println!("Writing: {}", text),
        Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
    }
}
```

## Ownership, Borrowing and Memory

### Learning Outcomes:
- Able to build reusable abstractions
- Manage memory safely
- Implement concurrent programs
- Understand stack vs heap allocation

### Topics:

#### Ownership Explained Simply
Ownership is Rust's most unique feature, enabling memory safety without garbage collection:

```rust
// Stack allocated (Copy trait)
let x = 5;
let y = x; // x is copied to y, both are valid

// Heap allocated (Move semantics)
let s1 = String::from("hello");
let s2 = s1; // s1 is moved to s2, s1 is invalidated
```

#### Stack vs Heap
- **Stack**: Fast allocation/deallocation, Last-In-First-Out, fixed-size data
- **Heap**: Dynamic allocation, slower access, flexible-size data

```rust
// Stack allocation
let x = 5; // Stored on stack

// Heap allocation
let s = String::from("hello"); // String data stored on heap
```

#### Move Semantics
When a value is assigned to a new variable, it's moved (not copied) if the type doesn't implement the Copy trait:

```rust
fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec1; // vec1 is moved to vec2
    
    // println!("{:?}", vec1); // Compile error: vec1 no longer valid
    println!("{:?}", vec2); // This works
}
```

#### Borrowing Rules
1. At any given time, you can have either:
   - One mutable reference, OR
   - Any number of immutable references
2. References must always be valid

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s; // First immutable borrow
    let r2 = &s; // Second immutable borrow
    // let r3 = &mut s; // Error: Cannot borrow as mutable while immutable borrows exist
    
    println!("{} and {}", r1, r2); // Last use of immutable borrows
    
    let r3 = &mut s; // Now we can borrow as mutable
    println!("{}", r3);
}
```

#### Lifetimes (Simple Cases)
Lifetimes ensure references are valid for as long as needed:

```rust
// Function signature with explicit lifetimes
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
```

#### Slices as Views
Slices provide a view into a contiguous sequence of elements:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    
    let slice = &a[1..3]; // Slice containing [2, 3]
    
    println!("Slice: {:?}", slice);
}
```

#### Hands-on Projects:
1. **Ownership Exercises**: Practice move semantics with different data types
2. **Mini-Grep**: Text search utility demonstrating borrowing concepts
3. **String Analyzer**: Process text files using string slices and ownership

## Data Structures for Systems

### Topics:

#### Arrays, Slices, Vectors
Different ways to store collections of data:

```rust
// Arrays: Fixed size, stack allocated
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let arr_init = [0; 10]; // Array of 10 zeros

// Slices: Dynamic view into data
let slice = &arr[1..3];

// Vectors: Dynamic arrays, heap allocated
let mut vec = Vec::new();
vec.push(1);
vec.push(2);

let vec_macro = vec![1, 2, 3, 4, 5];
```

#### Strings (&str vs String)
Two string types for different use cases:

```rust
// &str: String slice, usually string literals
let str_slice: &str = "Hello, world!";

// String: Owned, growable string type
let mut owned_string = String::new();
owned_string.push_str("Hello, ");
owned_string.push_str("world!");

// Converting between them
let str_from_string: &str = &owned_string; // Coerce String to &str
let string_from_str: String = str_slice.to_string(); // Convert &str to String
```

#### Structs & Enums
Custom data types:

```rust
// Structs
struct Point {
    x: f64,
    y: f64,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let width = (self.bottom_right.x - self.top_left.x).abs();
        let height = (self.top_left.y - self.bottom_right.y).abs();
        width * height
    }
}

// Enums
enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

#### Traits for Behavior
Traits define shared behavior:

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

#### Pattern Matching
Advanced pattern matching techniques:

```rust
enum Color {
    Red,
    Blue,
    Green,
    Rgb(u8, u8, u8),
    Hsv { hue: u8, saturation: u8, value: u8 },
}

fn describe_color(color: Color) {
    match color {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
        Color::Rgb(r, g, b) => println!("RGB({}, {}, {})", r, g, b),
        Color::Hsv { hue, saturation, value } => {
            println!("HSV({}, {}, {})", hue, saturation, value)
        }
    }
}
```

#### Hands-on Projects:
1. **Vector Wrapper**: Create a custom vector-like data structure
2. **Log Parser**: Parse and analyze system log files

## Introduction to Unsafe Rust

### Topics:

#### What Unsafe Means
Unsafe Rust allows you to bypass some of Rust's safety guarantees:

```rust
unsafe fn dangerous() {
    // Potentially unsafe operations
}

fn main() {
    unsafe {
        // Dereference raw pointers
        // Call unsafe functions
        // Access or modify mutable static variables
        // Implement unsafe traits
        dangerous();
    }
}
```

#### Raw Pointers
Raw pointers can be null and don't have automatic memory management:

```rust
fn main() {
    let mut num = 5;
    
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
```

#### Alias Rules
Unsafe code must still respect aliasing rules manually:

```rust
unsafe fn modify_both(x: *mut i32, y: *mut i32) {
    *x += 10;
    *y += 10;
    // Safe as long as x and y point to different memory locations
}
```

#### repr(C) Basics
Control struct layout for FFI compatibility:

```rust
#[repr(C)]
struct Point {
    x: f64,
    y: f64,
}
// Guaranteed to have C-compatible memory layout
```

#### Calling a Simple C Function
Using external C functions:

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

## Concurrency Basics

### Topics:

#### Threads
Creating and managing threads in Rust:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap();
}
```

#### Arc for Shared Ownership
Atomic Reference Counting for sharing data between threads:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Result: {}", *counter.lock().unwrap());
}
```

#### Mutex Locking Rules
Mutexes ensure thread-safe access to data:

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);
    
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } // Lock is automatically released here
    
    println!("m = {:?}", m);
}
```

#### Channels & Message Passing
Communicating between threads safely:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    for received in rx {
        println!("Got: {}", received);
    }
}
```

## Filesystem & OS Interaction

### Topics:

#### Read/Write Files
Basic file operations:

```rust
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn read_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_file(filename: &str, contents: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}
```

#### Directory Traversal
Navigating directory structures:

```rust
use std::fs;
use std::io;

fn visit_dirs(dir: &std::path::Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path)?;
            } else {
                println!("{}", path.display());
            }
        }
    }
    Ok(())
}
```

#### Metadata (Size, Permissions)
Accessing file metadata:

```rust
use std::fs;

fn file_metadata(path: &str) -> std::io::Result<()> {
    let metadata = fs::metadata(path)?;
    
    println!("File size: {} bytes", metadata.len());
    println!("Permissions: {:?}", metadata.permissions());
    println!("Modified: {:?}", metadata.modified());
    
    Ok(())
}
```

#### Running Commands
Executing system commands:

```rust
use std::process::Command;

fn main() -> std::io::Result<()> {
    let output = Command::new("ls")
        .args(&["-l", "-a"])
        .output()?;
    
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    
    Ok(())
}
```

#### System Calls
Interfacing with the operating system:

```rust
use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn strlen(cs: *const c_char) -> usize;
}

fn main() {
    let c_string = CString::new("Hello, world!").unwrap();
    
    unsafe {
        let len = strlen(c_string.as_ptr());
        println!("String length: {}", len);
    }
}
```

#### Hands-on Projects:
1. **Mini Shell**: Create a simple command-line shell
2. **File Sync Tool**: Synchronize files between directories

## Networking Basics

### Topics:

#### TCP vs UDP
Understanding transport layer protocols:

```rust
// TCP: Connection-oriented, reliable
// UDP: Connectionless, unreliable but faster

use std::net::{TcpListener, TcpStream, UdpSocket};
use std::io::prelude::*;
```

#### Sockets
Creating network connections:

```rust
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

// TCP Server
fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;
    stream.write(&buffer)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream).unwrap();
    }
    Ok(())
}
```

#### Blocking IO
Handling blocking network operations:

```rust
use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    
    stream.write(b"Hello, server!")?;
    stream.shutdown(Shutdown::Write)?;
    
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
    
    Ok(())
}
```

#### Simple HTTP Concepts
Basic HTTP client implementation:

```rust
use std::net::TcpStream;
use std::io::{BufRead, BufReader, Write};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("httpbin.org:80")?;
    
    let request = "GET /get HTTP/1.1\r\nHost: httpbin.org\r\nConnection: close\r\n\r\n";
    stream.write(request.as_bytes())?;
    
    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();
    
    while reader.read_line(&mut buffer)? > 0 {
        print!("{}", buffer);
        buffer.clear();
    }
    
    Ok(())
}
```

#### Packet Structure Basics
Understanding network packet components:

```rust
// Simplified representation of network packets
struct EthernetFrame {
    destination_mac: [u8; 6],
    source_mac: [u8; 6],
    ethertype: u16,
    payload: Vec<u8>,
}

struct IpPacket {
    version: u8,
    header_length: u8,
    total_length: u16,
    source_ip: [u8; 4],
    destination_ip: [u8; 4],
    payload: Vec<u8>,
}
```

#### Hands-on Projects:
1. **TCP Echo Server**: Simple server that echoes received messages
2. **Mini HTTP Client**: Basic HTTP client for making GET requests

## Case Study Projects

### Objective:
Apply all learned concepts in real-world projects and build end-to-end systems tools.

### Goals:

#### HTTP Server
Create a multithreaded HTTP server capable of serving static files and handling REST API requests:

```rust
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::fs::File;
use std::thread;

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    let request_line = &http_request[0];
    
    let (status_line, filename) = if request_line.starts_with("GET / ") {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    
    let contents = std::fs::read_to_string(filename).unwrap();
    let length = contents.len();
    
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line, length, contents
    );
    
    stream.write_all(response.as_bytes()).unwrap();
    Ok(())
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        thread::spawn(|| {
            handle_connection(stream).unwrap();
        });
    }
}
```

#### Key-Value Store
Implement a simple in-memory key-value store with persistence:

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

type KvStore = Arc<Mutex<HashMap<String, String>>>;

fn set(store: &KvStore, key: String, value: String) {
    let mut map = store.lock().unwrap();
    map.insert(key, value);
}

fn get(store: &KvStore, key: &str) -> Option<String> {
    let map = store.lock().unwrap();
    map.get(key).cloned()
}

fn save_to_disk(store: &KvStore, filename: &str) -> std::io::Result<()> {
    let map = store.lock().unwrap();
    let file = File::create(filename)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &*map)?;
    Ok(())
}
```

#### Packet Sniffer
Create a network packet analyzer that captures and displays network traffic:

```rust
// This would require unsafe code and platform-specific APIs
// Simplified example showing structure
use std::net::UdpSocket;

struct PacketSniffer {
    socket: UdpSocket,
}

impl PacketSniffer {
    fn new() -> std::io::Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        // Configure socket for promiscuous mode (platform-specific)
        Ok(PacketSniffer { socket })
    }
    
    fn capture_packets(&self) -> std::io::Result<()> {
        let mut buffer = [0; 1500]; // Typical MTU size
        loop {
            let (size, _) = self.socket.recv_from(&mut buffer)?;
            self.analyze_packet(&buffer[..size]);
        }
    }
    
    fn analyze_packet(&self, packet: &[u8]) {
        // Parse Ethernet frame header
        if packet.len() >= 14 {
            let ethertype = u16::from_be_bytes([packet[12], packet[13]]);
            println!("Ethernet Type: 0x{:04x}", ethertype);
            
            // Parse IP header if it's an IP packet
            if ethertype == 0x0800 { // IPv4
                self.parse_ipv4_header(&packet[14..]);
            }
        }
    }
    
    fn parse_ipv4_header(&self, ip_data: &[u8]) {
        if ip_data.len() >= 20 {
            let version = (ip_data[0] >> 4) & 0x0F;
            let ihl = ip_data[0] & 0x0F;
            let protocol = ip_data[9];
            
            println!("IP Version: {}", version);
            println!("Header Length: {} words", ihl);
            println!("Protocol: {}", protocol);
        }
    }
}
```

#### Mini Container
Build a lightweight container runtime similar to Docker:

```rust
use std::process::Command;
use std::fs;

struct Container {
    id: String,
    rootfs: String,
    pid: Option<u32>,
}

impl Container {
    fn new(id: String, rootfs: String) -> Self {
        Container {
            id,
            rootfs,
            pid: None,
        }
    }
    
    fn create_rootfs(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.rootfs)?;
        // Mount necessary filesystems (proc, sys, dev)
        Ok(())
    }
    
    fn run(&mut self, command: &[&str]) -> std::io::Result<()> {
        self.create_rootfs()?;
        
        // Use chroot or similar mechanism to isolate the process
        let output = Command::new("chroot")
            .arg(&self.rootfs)
            .args(command)
            .output()?;
        
        self.pid = Some(output.id());
        println!("Container started with PID: {:?}", self.pid);
        
        Ok(())
    }
}
```

#### REST API Server with Tokio and Axum
Create a modern, asynchronous RESTful API server for managing resources with CRUD operations using Tokio and Axum:

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::net::TcpListener;

// Data model
#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// Application state
#[derive(Debug, Clone)]
struct AppState {
    users: Arc<RwLock<HashMap<u32, User>>>,
}

// Request/Response types
#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct UpdateUserRequest {
    name: Option<String>,
    email: Option<String>,
}

// Handlers
async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = state.users.read().unwrap();
    let user_list: Vec<User> = users.values().cloned().collect();
    Json(user_list)
}

async fn get_user(
    Path(user_id): Path<u32>,
    State(state): State<AppState>,
) -> Result<Json<User>, StatusCode> {
    let users = state.users.read().unwrap();
    match users.get(&user_id) {
        Some(user) => Ok(Json(user.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, Json<User>) {
    let mut users = state.users.write().unwrap();
    
    // Generate new ID (in production, use a proper ID generator)
    let new_id = users.len() as u32 + 1;
    
    let user = User {
        id: new_id,
        name: payload.name,
        email: payload.email,
    };
    
    users.insert(new_id, user.clone());
    
    (StatusCode::CREATED, Json(user))
}

async fn update_user(
    Path(user_id): Path<u32>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    let mut users = state.users.write().unwrap();
    
    if let Some(user) = users.get_mut(&user_id) {
        if let Some(name) = payload.name {
            user.name = name;
        }
        if let Some(email) = payload.email {
            user.email = email;
        }
        Ok(Json(user.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn delete_user(
    Path(user_id): Path<u32>,
    State(state): State<AppState>,
) -> Result<StatusCode, StatusCode> {
    let mut users = state.users.write().unwrap();
    
    if users.remove(&user_id).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize state with sample data
    let mut users = HashMap::new();
    users.insert(1, User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    });
    users.insert(2, User {
        id: 2,
        name: "Bob".to_string(),
        email: "bob@example.com".to_string(),
    });
    
    let app_state = AppState {
        users: Arc::new(RwLock::new(users)),
    };

    // Build router
    let app = Router::new()
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .with_state(app_state);

    // Run server
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("REST API server running on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

To run this modern REST API server, you'll need to add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

This modern REST API example demonstrates:
- Asynchronous programming with Tokio
- Modern web framework with Axum
- Type-safe routing and extraction
- JSON serialization/deserialization
- Thread-safe shared state using Arc and RwLock
- CRUD operations (Create, Read, Update, Delete)
- Proper HTTP status codes
- Error handling with Result types

You can interact with this API using curl commands:
```bash
# Get all users
curl http://127.0.0.1:3000/users

# Get a specific user
curl http://127.0.0.1:3000/users/1

# Create a new user
curl -X POST http://127.0.0.1:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Charlie","email":"charlie@example.com"}'

# Update a user
curl -X PUT http://127.0.0.1:3000/users/1 \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice Smith","email":"alice.smith@example.com"}'

# Delete a user
curl -X DELETE http://127.0.0.1:3000/users/1
```

Benefits of using Tokio and Axum:
- **Asynchronous Performance**: Non-blocking I/O for high concurrency
- **Type Safety**: Compile-time guarantees for routes and handlers
- **Ergonomic APIs**: Clean, expressive code with minimal boilerplate
- **Built-in Features**: Middleware, extractors, and error handling
- **Ecosystem Integration**: Works seamlessly with the broader Rust async ecosystem

These case studies provide hands-on experience with real-world systems programming challenges while applying all the concepts covered in the training program.