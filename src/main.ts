import { ImageService } from "./app/services/ImageService";
import { AppState } from "./app/state/AppState";

// Initialize services
const imageService = new ImageService();
const appState = new AppState();

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    try {
      const message = await imageService.greet(greetInputEl.value);
      greetMsgEl.textContent = message;
      console.log("App state initialized:", appState);
    } catch (error) {
      greetMsgEl.textContent = `Error: ${error}`;
    }
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  console.log("Transform Images App initialized!");
  console.log("Architecture: Clean + Hexagonal");
});
