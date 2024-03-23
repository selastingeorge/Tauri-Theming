import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { invoke } from "@tauri-apps/api/core";

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  greetingMessage = "";
  public isDarkModeEnabled:boolean = true;

  public async toggleTheme() {
    let theme = (this.isDarkModeEnabled)?"light":"dark";
    await invoke('set_theme', { theme: theme });
    this.isDarkModeEnabled = !this.isDarkModeEnabled;
  }
}