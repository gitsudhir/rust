<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { getMatches } from "@tauri-apps/plugin-cli";

  let cliArgs = $state({});
  let cliError = $state("");
  
  // File operation variables
  let filePath = $state("");
  let fileContent = $state("");
  let operationResult = $state("");

  // Parse CLI arguments when the app starts
  async function parseCliArgs() {
    console.log('TextEditor initializing...');
    try {
      cliArgs = await getMatches();
      console.log('CLI Matches:', cliArgs);
      
      if (cliArgs) {
        displayCliInfo();
        await loadFileContent();
      } else {
        cliError = "Failed to initialize CLI service. CLI arguments may not be available.";
      }
    } catch (error) {
      console.error("Error parsing CLI arguments:", error);
      cliError = `Failed to parse CLI arguments: ${error}`;
    }
  }
  
  async function loadFileContent() {
    console.log('Loading file content...');
    try {
      const content = await getFileContent();
      console.log('File content loaded:', content);
      if (content !== null) {
        fileContent = content;
        filePath = getFileArgument() || "";
      }
    } catch (error) {
      console.error('Error loading file content:', error);
      cliError = `Failed to read file content: ${error}`;
    }
  }
  
  function displayCliInfo() {
    console.log('Displaying CLI info...');
    if (cliArgs) {
      const fileArg = getFileArgument();
      const outputArg = getOutputArgument();
      const isVerbose = isVerboseMode();
      const theme = getTheme();
      const subcommand = getSubcommand();
      
      operationResult = `File: ${fileArg || 'Not specified'}\n` +
                     `Output: ${outputArg || 'Not specified'}\n` +
                     `Verbose: ${isVerbose ? 'Yes' : 'No'}\n` +
                     `Theme: ${theme || 'Not specified'}\n` +
                     (subcommand ? `Subcommand: ${subcommand}\n` : '');
    } else {
      operationResult = "No CLI arguments provided.";
    }
    console.log('CLI Info:', operationResult);
  }
  
  function getFileArgument(): string | null {
    if (cliArgs && cliArgs.args && cliArgs.args.file) {
      return cliArgs.args.file.value;
    }
    return null;
  }
  
  function getOutputArgument(): string | null {
    if (cliArgs && cliArgs.args && cliArgs.args.output) {
      return cliArgs.args.output.value;
    }
    return null;
  }
  
  function isVerboseMode(): boolean {
    if (cliArgs && cliArgs.args && cliArgs.args.verbose) {
      return cliArgs.args.verbose.value === true;
    }
    return false;
  }
  
  function getTheme(): string | null {
    if (cliArgs && cliArgs.args && cliArgs.args.theme) {
      return cliArgs.args.theme.value;
    }
    return null;
  }
  
  function getAllArguments(): Record<string, any> {
    if (cliArgs && cliArgs.args) {
      const args: Record<string, any> = {};
      for (const [key, value] of Object.entries(cliArgs.args)) {
        args[key] = (value as any).value;
      }
      return args;
    }
    return {};
  }
  
  function hasArgument(argName: string): boolean {
    return cliArgs && 
           cliArgs.args && 
           argName in cliArgs.args;
  }
  
  function getArgumentValue(argName: string): any {
    if (hasArgument(argName)) {
      return cliArgs.args[argName].value;
    }
    return undefined;
  }
  
  function getSubcommand(): string | null {
    if (cliArgs && cliArgs.subcommand) {
      return cliArgs.subcommand.name;
    }
    return null;
  }
  
  function getSubcommandMatches(): any {
    if (cliArgs && cliArgs.subcommand?.matches) {
      return cliArgs.subcommand.matches;
    }
    return null;
  }
  
  async function readFileContent(filePath: string): Promise<string> {
    try {
      console.log('Attempting to read file via Rust command:', filePath);
      const content = await invoke<string>('read_file_content', { filePath });
      console.log('File content read successfully via Rust command');
      return content;
    } catch (error) {
      console.error(`Failed to read file ${filePath}:`, error);
      throw error;
    }
  }
  
  async function getFileContent(): Promise<string | null> {
    const filePath = getFileArgument();
    console.log('File path from CLI:', filePath);
    if (filePath) {
      try {
        return await readFileContent(filePath);
      } catch (error) {
        console.error(`Error reading file content:`, error);
        return null;
      }
    }
    console.log('No file path provided');
    return null;
  }

  // File operations
  async function readFile(path: string) {
    try {
      fileContent = await invoke("read_file", { path }) as string;
      filePath = path;
      operationResult = `Successfully read file: ${path}`;
    } catch (error) {
      operationResult = `Error reading file: ${error}`;
    }
  }
  
  async function writeFile() {
    try {
      await invoke("write_file", { path: filePath, content: fileContent });
      operationResult = `Successfully wrote to file: ${filePath}`;
    } catch (error) {
      operationResult = `Error writing file: ${error}`;
    }
  }
  
  async function createDirectory() {
    try {
      await invoke("create_directory", { path: filePath });
      operationResult = `Successfully created directory: ${filePath}`;
    } catch (error) {
      operationResult = `Error creating directory: ${error}`;
    }
  }
  
  async function deleteFile() {
    try {
      await invoke("delete_file", { path: filePath });
      operationResult = `Successfully deleted: ${filePath}`;
      fileContent = "";
    } catch (error) {
      operationResult = `Error deleting file/directory: ${error}`;
    }
  }
  
  async function checkFileExists() {
    try {
      const exists = await invoke("file_exists", { path: filePath });
      operationResult = `File ${filePath} ${exists ? 'exists' : 'does not exist'}`;
    } catch (error) {
      operationResult = `Error checking file existence: ${error}`;
    }
  }

  // Call the function when the component mounts
  parseCliArgs();
</script>

<main class="container">
  <h1>AI Text Editor</h1>
  
  <div class="editor-container">
    <div class="file-info">
      <input 
        id="file-path" 
        placeholder="Enter file path..." 
        bind:value={filePath} 
      />
      <div class="file-buttons">
        <button onclick={() => readFile(filePath)}>Open</button>
        <button onclick={writeFile}>Save</button>
        <button onclick={checkFileExists}>Check</button>
      </div>
    </div>
    
    <textarea 
      id="file-content" 
      placeholder="File content will appear here..." 
      bind:value={fileContent}
    ></textarea>
    
    <div class="status-bar">
      <span class="operation-result">{operationResult}</span>
    </div>
  </div>
  
  <div class="cli-info" class:hide={!cliArgs}>
    <h2>CLI Information</h2>
    <pre>{operationResult}</pre>
  </div>

  <div class="cli-error" class:hide={!cliError}>
    <h2>CLI Error</h2>
    <p>{cliError}</p>
  </div>
</main>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding: 20px;
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.editor-container {
  display: flex;
  flex-direction: column;
  flex: 1;
  gap: 10px;
}

.file-info {
  display: flex;
  gap: 10px;
  align-items: center;
}

#file-path {
  flex: 1;
  padding: 8px;
  border-radius: 4px;
  border: 1px solid #ccc;
}

.file-buttons {
  display: flex;
  gap: 5px;
}

button {
  padding: 8px 12px;
  border-radius: 4px;
  border: 1px solid #ccc;
  background-color: #fff;
  cursor: pointer;
  font-size: 14px;
}

button:hover {
  background-color: #f0f0f0;
}

#file-content {
  flex: 1;
  padding: 12px;
  border-radius: 4px;
  border: 1px solid #ccc;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 14px;
  resize: none;
}

.status-bar {
  padding: 8px;
  background-color: #f0f0f0;
  border-radius: 4px;
  font-size: 12px;
  color: #666;
}

.cli-info, .cli-error {
  margin-top: 20px;
  padding: 10px;
  border-radius: 4px;
}

.cli-info {
  background-color: #e8f4fd;
}

.cli-error {
  background-color: #ffebee;
  color: #c62828;
}

.cli-info h2, .cli-error h2 {
  margin-top: 0;
  font-size: 16px;
}

.hide {
  display: none;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #1a1a1a;
  }
  
  #file-path, #file-content {
    background-color: #2d2d2d;
    color: #f6f6f6;
    border-color: #444;
  }
  
  button {
    background-color: #2d2d2d;
    color: #f6f6f6;
    border-color: #444;
  }
  
  button:hover {
    background-color: #3d3d3d;
  }
  
  .status-bar {
    background-color: #2d2d2d;
    color: #aaa;
  }
  
  .cli-info {
    background-color: #1a3d5d;
  }
  
  .cli-error {
    background-color: #5d1a1a;
  }
}
</style>