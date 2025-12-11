<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { getMatches } from "@tauri-apps/plugin-cli";

  let cliArgs = $state({});
  let cliError = $state("");
  
  // File operation variables
  let filePath = $state("");
  let fileContent = $state("");
  let operationResult = $state("");
  
  // AI variables
  let aiPrompt = $state("");
  let isAiGenerating = $state(false);

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
  
  // AI operations
  async function generateAiText() {
    if (!aiPrompt.trim()) {
      operationResult = "Please enter a prompt for AI generation";
      return;
    }
    
    isAiGenerating = true;
    operationResult = "Generating AI content...";
    
    try {
      const aiResponse = await invoke<string>("generate_ai_text", { prompt: aiPrompt });
      fileContent = aiResponse;
      operationResult = "AI content generated successfully";
    } catch (error) {
      operationResult = `AI generation failed: ${error}`;
      console.error("AI generation error:", error);
      // Show a more detailed error in the console for debugging
      console.error("Full error details:", JSON.stringify(error, null, 2));
    } finally {
      isAiGenerating = false;
    }
  }
  
  async function appendAiText() {
    if (!aiPrompt.trim()) {
      operationResult = "Please enter a prompt for AI generation";
      return;
    }
    
    isAiGenerating = true;
    operationResult = "Generating AI content...";
    
    try {
      const aiResponse = await invoke<string>("generate_ai_text", { prompt: aiPrompt });
      fileContent += "\n\n" + aiResponse;
      operationResult = "AI content appended successfully";
    } catch (error) {
      operationResult = `AI generation failed: ${error}`;
      console.error("AI generation error:", error);
      // Show a more detailed error in the console for debugging
      console.error("Full error details:", JSON.stringify(error, null, 2));
    } finally {
      isAiGenerating = false;
    }
  }

  // Call the function when the component mounts
  parseCliArgs();
</script>

<main class="container">
  <header class="app-header">
    <h1>AI Text Editor</h1>
    <p>Build by sudhirkumar.in</p>
  </header>
  
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
    
    <div class="ai-section">
      <div class="ai-input-container">
        <input 
          id="ai-prompt" 
          placeholder="Enter AI prompt (e.g., 'Write a short story about a robot learning to paint')..." 
          bind:value={aiPrompt}
          disabled={isAiGenerating}
        />
        <div class="ai-buttons">
          <button onclick={generateAiText} disabled={isAiGenerating}>
            {isAiGenerating ? "Generating..." : "Generate"}
          </button>
          <button onclick={appendAiText} disabled={isAiGenerating}>
            {isAiGenerating ? "Generating..." : "Append"}
          </button>
        </div>
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
  background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.app-header {
  text-align: center;
  padding: 8px 0;
  margin-bottom: 10px;
  background: linear-gradient(90deg, #6a11cb 0%, #2575fc 100%);
  border-radius: 8px;
  color: white;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.app-header h1 {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
}

.app-header p {
  margin: 2px 0 0 0;
  font-size: 0.8rem;
  opacity: 0.9;
}

.editor-container {
  display: flex;
  flex-direction: column;
  flex: 1;
  gap: 10px;
  background: rgba(255, 255, 255, 0.8);
  border-radius: 8px;
  padding: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
}

.file-info, .ai-input-container {
  display: flex;
  gap: 10px;
  align-items: center;
}

.file-info {
  padding: 5px 0;
}

#file-path, #ai-prompt {
  flex: 1;
  padding: 10px;
  border-radius: 6px;
  border: 2px solid #e1e5f0;
  background-color: #ffffff;
  font-size: 14px;
  box-shadow: inset 0 1px 3px rgba(0, 0, 0, 0.1);
  transition: border-color 0.3s;
}

#file-path:focus, #ai-prompt:focus {
  outline: none;
  border-color: #6a11cb;
  box-shadow: 0 0 0 3px rgba(106, 17, 203, 0.1);
}

.file-buttons, .ai-buttons {
  display: flex;
  gap: 8px;
}

button {
  padding: 8px 12px;
  border-radius: 6px;
  border: none;
  background: linear-gradient(90deg, #6a11cb 0%, #2575fc 100%);
  color: white;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: transform 0.2s, box-shadow 0.2s;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
}

button:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
}

button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none;
}

button:active:not(:disabled) {
  transform: translateY(0);
}

#file-content {
  flex: 1;
  padding: 12px;
  border-radius: 6px;
  border: 2px solid #e1e5f0;
  background-color: #ffffff;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 14px;
  resize: none;
  box-shadow: inset 0 1px 3px rgba(0, 0, 0, 0.1);
  transition: border-color 0.3s;
}

#file-content:focus {
  outline: none;
  border-color: #6a11cb;
  box-shadow: 0 0 0 3px rgba(106, 17, 203, 0.1);
}

.status-bar {
  padding: 10px;
  background: linear-gradient(90deg, #f5f7fa 0%, #c3cfe2 100%);
  border-radius: 6px;
  font-size: 12px;
  color: #4a5568;
  font-weight: 500;
  border: 1px solid #e2e8f0;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.ai-section {
  padding: 15px 0;
  border-top: 1px solid #e2e8f0;
  border-bottom: 1px solid #e2e8f0;
  background: linear-gradient(135deg, #f0f4ff 0%, #e6f0ff 100%);
  border-radius: 6px;
  margin: 5px 0;
  padding: 12px;
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
    background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
  }
  
  .container {
    background-color: #1a1a2e;
  }
  
  .app-header {
    background: linear-gradient(90deg, #0f0c29 0%, #302b63 100%);
  }
  
  .editor-container {
    background: rgba(30, 30, 46, 0.8);
    color: #f6f6f6;
  }
  
  #file-path, #file-content, #ai-prompt {
    background-color: #2d2d42;
    color: #f6f6f6;
    border-color: #44475a;
  }
  
  #file-path:focus, #ai-prompt:focus, #file-content:focus {
    border-color: #6a11cb;
    box-shadow: 0 0 0 3px rgba(106, 17, 203, 0.2);
  }
  
  button {
    background: linear-gradient(90deg, #6a11cb 0%, #2575fc 100%);
    color: #f6f6f6;
    border: none;
  }
  
  button:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.4);
  }
  
  .status-bar {
    background: linear-gradient(90deg, #2d2d42 0%, #3a3a5a 100%);
    color: #e2e8f0;
    border-color: #44475a;
  }
  
  .ai-section {
    background: linear-gradient(135deg, #252540 0%, #1f1f3a 100%);
    border-color: #44475a;
  }
  
  .cli-info {
    background-color: #1a3d5d;
  }
  
  .cli-error {
    background-color: #5d1a1a;
  }
}
</style>