<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { getMatches } from "@tauri-apps/plugin-cli";

  let name = $state("");
  let greetMsg = $state("");
  let cliArgs = $state({});
  let cliError = $state("");
  
  // File operation variables
  let filePath = $state("");
  let fileContent = $state("");
  let operationResult = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }

  // Parse CLI arguments when the app starts
  async function parseCliArgs() {
    console.log('AppComponent initializing...');
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
      const subcommandMatches = getSubcommandMatches();
      
      operationResult = `File: ${fileArg || 'Not specified'}\n` +
                     `Output: ${outputArg || 'Not specified'}\n` +
                     `Verbose: ${isVerbose ? 'Yes' : 'No'}\n` +
                     `Theme: ${theme || 'Not specified'}\n` +
                     (subcommand ? `Subcommand: ${subcommand}\n` : '') +
                     (subcommandMatches ? `Subcommand Args: ${JSON.stringify(subcommandMatches, null, 2)}\n` : '') +
                     `All Args: ${JSON.stringify(getAllArguments(), null, 2)}`;
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
  <h1>Welcome to Tauri + Svelte</h1>

  <div class="row">
    <a href="https://vite.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte-kit" alt="SvelteKit Logo" />
    </a>
  </div>
  <p>Click on the Tauri, Vite, and SvelteKit logos to learn more.</p>

  <div class="cli-info" class:hide={!cliArgs}>
    <h2>CLI Information</h2>
    <pre>{operationResult}</pre>
  </div>

  <div class="file-content" class:hide={!fileContent}>
    <h2>File Content</h2>
    <pre>{fileContent}</pre>
  </div>

  <div class="cli-error" class:hide={!cliError}>
    <h2>CLI Error</h2>
    <p>{cliError}</p>
  </div>

  <form class="row" onsubmit={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>
  
  <!-- File Operations Section -->
  <div class="file-operations">
    <h2>File Operations</h2>
    <div class="file-input-row">
      <input 
        id="file-path" 
        placeholder="Enter file path..." 
        bind:value={filePath} 
      />
    </div>
    
    <div class="file-buttons">
      <button onclick={() => readFile(filePath)}>Read File</button>
      <button onclick={checkFileExists}>Check Exists</button>
      <button onclick={createDirectory}>Create Dir</button>
      <button onclick={deleteFile}>Delete</button>
    </div>
    
    <textarea 
      id="file-content" 
      placeholder="File content will appear here..." 
      bind:value={fileContent}
    ></textarea>
    
    <div class="file-buttons">
      <button onclick={writeFile}>Write File</button>
    </div>
    
    <p class="operation-result">{operationResult}</p>
  </div>
</main>

<style>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.svelte-kit:hover {
  filter: drop-shadow(0 0 2em #ff3e00);
}

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
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

.cli-info {
  margin-top: 2rem;
  padding: 1rem;
  background-color: #f0f0f0;
  border-radius: 8px;
  text-align: left;
}

.cli-info h2 {
  margin-top: 0;
}

.cli-info pre {
  background-color: #e0e0e0;
  padding: 1rem;
  border-radius: 4px;
  overflow-x: auto;
}

.file-operations {
  margin-top: 2rem;
  padding: 1rem;
  background-color: #f0f0f0;
  border-radius: 8px;
  text-align: left;
}

.file-operations h2 {
  margin-top: 0;
}

.file-input-row {
  margin-bottom: 1rem;
}

#file-path {
  width: 100%;
  margin-right: 0;
}

.file-buttons {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
  margin-bottom: 1rem;
}

#file-content {
  width: 100%;
  min-height: 150px;
  margin-bottom: 1rem;
  font-family: monospace;
  padding: 0.5rem;
}

.operation-result {
  font-style: italic;
  color: #666;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
  
  .cli-info {
    background-color: #3f3f3f;
  }
  
  .cli-info pre {
    background-color: #2f2f2f;
  }
  
  .file-operations {
    background-color: #3f3f3f;
  }
  
  .operation-result {
    color: #aaa;
  }
  
  .hide {
    display: none;
  }
}

</style>