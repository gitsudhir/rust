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
  
  // File explorer state
  let expandedFolders = $state(new Set<string>());
  let folderContents = $state({} as Record<string, Array<[string, boolean]>>);

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
      
      // Load file explorer
      await loadFileExplorer();
    } catch (error) {
      console.error("Error parsing CLI arguments:", error);
      cliError = `Failed to parse CLI arguments: ${error}`;
    }
  }
  
  async function loadFileExplorer() {
    try {
      // Get current directory
      const currentDir = ".";
      const entries = await listDirectoryContents(currentDir);
      renderFileTree(entries);
    } catch (error) {
      console.error("Error loading file explorer:", error);
      operationResult = `Error loading file explorer: ${error}`;
    }
  }
  
  async function listDirectoryContents(path: string): Promise<Array<[string, boolean]>> {
    try {
      const entries = await invoke<Array<[string, boolean]>>("list_directory_contents", { path });
      return entries;
    } catch (error) {
      console.error(`Failed to list directory contents for ${path}:`, error);
      throw error;
    }
  }
  
  async function toggleFolder(folderPath: string) {
    if (expandedFolders.has(folderPath)) {
      // Collapse folder
      expandedFolders.delete(folderPath);
    } else {
      // Expand folder
      try {
        expandedFolders.add(folderPath);
        const entries = await listDirectoryContents(folderPath);
        folderContents[folderPath] = entries;
      } catch (error) {
        console.error(`Failed to load contents of folder ${folderPath}:`, error);
        operationResult = `Failed to load folder contents: ${error}`;
        expandedFolders.delete(folderPath);
      }
    }
    // Re-render the file tree
    loadFileExplorer();
  }
  
  function renderFileTree(entries: Array<[string, boolean]>) {
    const fileTree = document.getElementById('file-tree');
    if (!fileTree) return;
    
    fileTree.innerHTML = '';
    
    // Get just the filename part if filePath contains a path
    let currentFileName = filePath;
    if (filePath && filePath.includes('/')) {
      currentFileName = filePath.split('/').pop() || filePath;
    }
    
    // File type icon mapping with comprehensive DevOps support
    const fileIcons: Record<string, string> = {
      // DevOps Configuration & Pipelines
      '.yaml': '⚙️', '.yml': '⚙️', '.json': '⚙️', '.xml': '⚙️',
      '.conf': '🔧', '.ini': '🔧', '.env': '🔐', '.toml': '⚙️',
      '.cfg': '🔧', '.config': '🔧',
      
      // DevOps Scripting & Automation
      '.sh': '쉘', '.ps1': '⚡', '.py': '🐍', '.rb': '💎',
      '.pl': '🐪', '.bat': '⚡', '.cmd': '⚡',
      
      // Infrastructure as Code (IaC) & Containers
      '.tf': '🏗️', '.tfvars': '🏗️', 'Dockerfile': '🐳', '.dockerignore': '🐳',
      'docker-compose.yml': '🐳', 'docker-compose.yaml': '🐳',
      
      // Cloud Provider Specific
      '.template': '☁️', // AWS CloudFormation
      '.ebextensions': '☁️', // AWS Elastic Beanstalk
      
      // Application/Code Related
      '.js': '📜', '.ts': '📜', '.jsx': '⚛️', '.tsx': '⚛️',
      '.html': '🌐', '.css': '🎨', '.scss': '🎨', '.sass': '🎨',
      '.go': '🐹', '.java': '☕', '.cs': '☪️', '.php': '🐘',
      '.cpp': '++', '.c': '🇨', '.rs': '🦀', '.kt': '📌',
      '.jar': '📦', '.war': '📦', '.ear': '📦',
      
      // Documentation & Data
      '.md': '📘', '.markdown': '📘', '.txt': '📝',
      '.csv': '📊', '.tsv': '📊', '.log': '📋',
      '.pdf': '📚', '.doc': '📘', '.docx': '📘',
      
      // Media files
      '.png': '🖼️', '.jpg': '🖼️', '.jpeg': '🖼️', '.gif': '🖼️',
      '.svg': '🎨', '.ico': '🌟', '.bmp': '🖼️',
      '.mp4': '🎬', '.avi': '🎬', '.mov': '🎬', '.mkv': '🎬',
      '.mp3': '🎵', '.wav': '🎵', '.flac': '🎵', '.aac': '🎵',
      
      // Web frameworks
      '.vue': '💚', '.svelte': '❤️', '.astro': '🌌', '.elm': '🇪',
      
      // Database
      '.sql': '🗄️', '.db': '🗄️', '.sqlite': '🗄️', '.dump': '🗄️',
      
      // Archives
      '.zip': '📦', '.rar': '📦', '.tar': '📦', '.gz': '📦',
      '.7z': '📦', '.bz2': '📦',
      
      // Executables & Binaries
      '.exe': '⚡', '.dll': '⚡', '.so': '⚡', '.app': '⚡',
      '.bin': '🔢', '.iso': '💿',
      
      // DevOps Tools & Platform Specific
      '.otd': '🧪', '.oti': '🧪', '.pts': '🧪',
      '.vss-extension.json': '🔌',
      
      // Nginx specific
      'nginx.conf': '🌐', '.nginx': '🌐',
      
      // Default
      'default': '📄',
      'folder': '📁',
      'folder-open': '📂'
    };
    
    // Get appropriate icon for a file based on its extension or name
    function getFileIcon(filename: string, isDirectory: boolean): string {
      if (isDirectory) {
        return fileIcons['folder'];
      }
      
      // Check for exact filename matches first (for special files like Dockerfile)
      if (fileIcons[filename]) {
        return fileIcons[filename];
      }
      
      // Check for extension-based matches
      const ext = '.' + filename.split('.').pop()?.toLowerCase();
      return fileIcons[ext] || fileIcons['default'];
    }
    
    // Helper function to render entries recursively
    function renderEntries(entries: Array<[string, boolean]>, parentPath: string = '') {
      return entries.map(([name, isDir]) => {
        const fullPath = parentPath ? `${parentPath}/${name}` : name;
        const li = document.createElement('li');
        li.className = 'file-item';
        li.dataset.name = fullPath;
        li.dataset.isDirectory = isDir.toString();
        
        // Highlight the currently opened file
        if (fullPath === currentFileName || name === currentFileName) {
          li.classList.add('active-file');
        }
        
        if (isDir) {
          // Create folder entry with expand/collapse functionality
          const fileEntry = document.createElement('div');
          fileEntry.className = 'file-entry folder';
          
          // Create expand/collapse indicator
          const indicatorSpan = document.createElement('span');
          indicatorSpan.className = 'folder-indicator';
          indicatorSpan.textContent = expandedFolders.has(fullPath) ? fileIcons['folder-open'] : fileIcons['folder'];
          indicatorSpan.style.cursor = 'pointer';
          indicatorSpan.style.marginRight = '4px';
          
          const nameSpan = document.createElement('span');
          nameSpan.className = 'file-name';
          nameSpan.textContent = name;
          nameSpan.style.cursor = 'pointer';
          
          fileEntry.appendChild(indicatorSpan);
          fileEntry.appendChild(nameSpan);
          li.appendChild(fileEntry);
          
          // Add click event to toggle folder
          li.addEventListener('click', (e) => {
            e.stopPropagation();
            toggleFolder(fullPath);
          });
          
          // If folder is expanded, render its contents
          if (expandedFolders.has(fullPath) && folderContents[fullPath]) {
            const subList = document.createElement('ul');
            subList.className = 'sub-folder';
            subList.style.marginLeft = '20px';
            subList.style.listStyle = 'none';
            subList.style.paddingLeft = '0';
            
            // Render sub-folder contents
            const subItems = renderEntries(folderContents[fullPath], fullPath);
            subItems.forEach(item => subList.appendChild(item));
            
            li.appendChild(subList);
          }
        } else {
          // Create file entry with type-specific icon
          const fileEntry = document.createElement('div');
          fileEntry.className = 'file-entry file';
          
          const iconSpan = document.createElement('span');
          iconSpan.className = 'file-icon';
          iconSpan.textContent = getFileIcon(name, false);
          iconSpan.style.cursor = 'pointer';
          iconSpan.style.marginRight = '4px';
          iconSpan.style.fontSize = '1rem';
          
          const nameSpan = document.createElement('span');
          nameSpan.className = 'file-name';
          nameSpan.textContent = name;
          nameSpan.style.cursor = 'pointer';
          nameSpan.style.fontSize = '0.9rem';
          nameSpan.style.color = '#4a5568';
          
          fileEntry.appendChild(iconSpan);
          fileEntry.appendChild(nameSpan);
          li.appendChild(fileEntry);
          
          // Add click event to open files
          li.addEventListener('click', () => {
            filePath = fullPath;
            readFile(fullPath);
            // Re-render to highlight the active file
            loadFileExplorer();
          });
        }
        
        return li;
      });
    }
    
    // Render root entries
    const items = renderEntries(entries);
    items.forEach(item => fileTree.appendChild(item));
  }
  
  async function loadFileContent() {
    console.log('Loading file content...');
    try {
      const content = await getFileContent();
      console.log('File content loaded:', content);
      if (content !== null) {
        fileContent = content;
        // Set filePath to just the filename, not the full path
        const fileArg = getFileArgument();
        if (fileArg) {
          filePath = fileArg.includes('/') ? fileArg.split('/').pop() || fileArg : fileArg;
        }
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
  
  <div class="main-layout">
    <aside class="sidebar">
      <div class="sidebar-header">
        <h3>Explorer</h3>
      </div>
      <ul class="file-tree" id="file-tree">
        <!-- File tree will be populated by JavaScript -->
      </ul>
    </aside>
    
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

.main-layout {
  display: flex;
  flex: 1;
  gap: 15px;
  overflow: hidden;
}

.sidebar {
  width: 250px;
  background: rgba(255, 255, 255, 0.8);
  border-radius: 8px;
  padding: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  overflow-y: auto;
  max-height: calc(100vh - 120px);
}

.sidebar-header {
  padding-bottom: 10px;
  border-bottom: 1px solid #e2e8f0;
  margin-bottom: 10px;
}

.sidebar-header h3 {
  margin: 0;
  color: #4a5568;
  font-size: 1rem;
}

.file-tree {
  list-style: none;
  padding: 0;
  margin: 0;
}

.folder-indicator {
  font-size: 1rem;
  cursor: pointer;
  margin-right: 4px;
}

.file-item {
  margin: 2px 0;
  border-radius: 4px;
  padding: 4px 8px;
  transition: background-color 0.2s;
  cursor: pointer;
}

.file-item:hover {
  background-color: rgba(106, 17, 203, 0.1);
}

.file-entry {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.folder-icon, .file-icon {
  font-size: 1rem;
  cursor: pointer;
}

.file-name {
  font-size: 0.9rem;
  color: #4a5568;
  cursor: pointer;
}

.folder .file-name {
  font-weight: 500;
}

.active-file {
  background-color: rgba(106, 17, 203, 0.2);
  font-weight: 500;
}

.active-file .file-name {
  color: #6a11cb;
}

.file-entry {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.folder-icon, .file-icon {
  font-size: 1rem;
  cursor: pointer;
}

.file-name {
  font-size: 0.9rem;
  color: #4a5568;
  cursor: pointer;
}

.folder .file-name {
  font-weight: 500;
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
  
  .main-layout {
    /* Inherits display: flex from light mode */
  }
  
  .sidebar {
    background: rgba(30, 30, 46, 0.8);
    color: #f6f6f6;
  }
  
  .sidebar-header {
    border-color: #44475a;
  }
  
  .sidebar-header h3 {
    color: #e2e8f0;
  }
  
  .folder-indicator {
    cursor: pointer;
  }
  
  .file-item {
    cursor: pointer;
  }
  
  .file-item:hover {
    background-color: rgba(106, 17, 203, 0.2);
    cursor: pointer;
  }
  
  .file-entry {
    cursor: pointer;
  }
  
  .file-name {
    color: #e2e8f0;
    cursor: pointer;
  }
  
  .folder-icon, .file-icon {
    cursor: pointer;
  }
  
  .active-file {
    background-color: rgba(106, 17, 203, 0.3);
    cursor: pointer;
  }
  
  .active-file .file-name {
    color: #a78bfa;
    cursor: pointer;
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