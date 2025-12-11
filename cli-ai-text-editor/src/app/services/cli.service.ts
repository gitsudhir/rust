import { Injectable } from '@angular/core';
import { getMatches } from '@tauri-apps/plugin-cli';
import { invoke } from '@tauri-apps/api/core';

@Injectable({
  providedIn: 'root'
})
export class CliService {
  private cliMatches: any = null;
  private initialized = false;

  async initialize(): Promise<boolean> {
    if (this.initialized) {
      return true;
    }

    try {
      this.cliMatches = await getMatches();
      this.initialized = true;
      console.log('CLI Matches:', this.cliMatches);
      return true;
    } catch (error) {
      console.error('Failed to get CLI matches:', error);
      return false;
    }
  }

  isInitialized(): boolean {
    return this.initialized;
  }

  getMatches(): any {
    return this.cliMatches;
  }

  getFileArgument(): string | null {
    if (this.cliMatches && this.cliMatches.args && this.cliMatches.args.file) {
      return this.cliMatches.args.file.value;
    }
    return null;
  }

  getOutputArgument(): string | null {
    if (this.cliMatches && this.cliMatches.args && this.cliMatches.args.output) {
      return this.cliMatches.args.output.value;
    }
    return null;
  }

  isVerbose(): boolean {
    if (this.cliMatches && this.cliMatches.args && this.cliMatches.args.verbose) {
      return this.cliMatches.args.verbose.value === true;
    }
    return false;
  }

  getTheme(): string | null {
    if (this.cliMatches && this.cliMatches.args && this.cliMatches.args.theme) {
      return this.cliMatches.args.theme.value;
    }
    return null;
  }

  /**
   * Get all CLI arguments as a key-value map
   */
  getAllArguments(): Record<string, any> {
    if (this.cliMatches && this.cliMatches.args) {
      const args: Record<string, any> = {};
      for (const [key, value] of Object.entries(this.cliMatches.args)) {
        args[key] = (value as any).value;
      }
      return args;
    }
    return {};
  }

  /**
   * Check if a specific argument was provided
   */
  hasArgument(argName: string): boolean {
    return this.cliMatches && 
           this.cliMatches.args && 
           argName in this.cliMatches.args;
  }

  /**
   * Get the value of a specific argument
   */
  getArgumentValue(argName: string): any {
    if (this.hasArgument(argName)) {
      return this.cliMatches.args[argName].value;
    }
    return undefined;
  }

  /**
   * Get CLI subcommand if any
   */
  getSubcommand(): string | null {
    if (this.cliMatches && this.cliMatches.subcommand) {
      return this.cliMatches.subcommand;
    }
    return null;
  }

  /**
   * Get subcommand matches if any
   */
  getSubcommandMatches(): any {
    if (this.cliMatches && this.cliMatches.subcommandMatches) {
      return this.cliMatches.subcommandMatches;
    }
    return null;
  }

  /**
   * Read the content of a file
   */
  async readFileContent(filePath: string): Promise<string> {
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

  /**
   * Get file content if a file argument was provided
   */
  async getFileContent(): Promise<string | null> {
    const filePath = this.getFileArgument();
    console.log('File path from CLI:', filePath);
    if (filePath) {
      try {
        return await this.readFileContent(filePath);
      } catch (error) {
        console.error(`Error reading file content:`, error);
        return null;
      }
    }
    console.log('No file path provided');
    return null;
  }
}