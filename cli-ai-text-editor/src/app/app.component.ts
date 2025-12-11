import { Component, OnInit } from "@angular/core";
import { RouterOutlet } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";
import { CliService } from "./services/cli.service";
import { NgIf } from "@angular/common";

@Component({
  selector: "app-root",
  imports: [RouterOutlet, NgIf],
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.css",
})
export class AppComponent implements OnInit {
  greetingMessage = "";
  cliInfo = "";
  cliError = "";
  fileContent = "";

  constructor(private cliService: CliService) {}

  async ngOnInit() {
    console.log('AppComponent initializing...');
    const initialized = await this.cliService.initialize();
    console.log('CLI Service initialized:', initialized);
    if (initialized) {
      this.displayCliInfo();
      await this.loadFileContent();
      this.applyTheme();
    } else {
      this.cliError = "Failed to initialize CLI service. CLI arguments may not be available.";
    }
  }

  async loadFileContent() {
    console.log('Loading file content...');
    try {
      const content = await this.cliService.getFileContent();
      console.log('File content loaded:', content);
      if (content !== null) {
        this.fileContent = content;
      }
    } catch (error) {
      console.error('Error loading file content:', error);
      this.cliError = `Failed to read file content: ${error}`;
    }
  }

  displayCliInfo() {
    console.log('Displaying CLI info...');
    const matches = this.cliService.getMatches();
    console.log('Matches:', matches);
    if (matches) {
      const fileArg = this.cliService.getFileArgument();
      const outputArg = this.cliService.getOutputArgument();
      const isVerbose = this.cliService.isVerbose();
      const theme = this.cliService.getTheme();
      const subcommand = this.cliService.getSubcommand();
      const subcommandMatches = this.cliService.getSubcommandMatches();
      
      this.cliInfo = `File: ${fileArg || 'Not specified'}\n` +
                     `Output: ${outputArg || 'Not specified'}\n` +
                     `Verbose: ${isVerbose ? 'Yes' : 'No'}\n` +
                     `Theme: ${theme || 'Not specified'}\n` +
                     (subcommand ? `Subcommand: ${subcommand}\n` : '') +
                     (subcommandMatches ? `Subcommand Args: ${JSON.stringify(subcommandMatches, null, 2)}\n` : '') +
                     `All Args: ${JSON.stringify(this.cliService.getAllArguments(), null, 2)}`;
    } else {
      this.cliInfo = "No CLI arguments provided.";
    }
    console.log('CLI Info:', this.cliInfo);
  }

  applyTheme() {
    const theme = this.cliService.getTheme();
    if (theme) {
      if (theme === 'dark') {
        document.body.classList.add('dark-theme');
      } else if (theme === 'light') {
        document.body.classList.remove('dark-theme');
      }
    }
  }

  greet(event: SubmitEvent, name: string): void {
    event.preventDefault();

    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    invoke<string>("greet", { name }).then((text) => {
      this.greetingMessage = text;
    });
  }
}