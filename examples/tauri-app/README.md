# Lingua Language Detector - Example App

This example app demonstrates all the features of the `tauri-plugin-lingua` plugin.

## Features Demonstrated

1. **Create Language Detector**
   - Create detector for all 75 supported languages
   - Create detector for specific languages using ISO 639-1 codes

2. **Detect Language**
   - Detect language from any text input
   - Returns ISO 639-1 language code

3. **Language Confidence**
   - Get confidence score for a specific language
   - View all confidence scores sorted by probability

4. **Sample Texts**
   - Pre-loaded sample texts in 8 different languages for quick testing

## Running the App

```bash
pnpm install
pnpm tauri dev
```

## Building the App

```bash
pnpm tauri build
```

## How to Use

1. **Step 1**: Click "Create Detector" to initialize a language detector
   - Choose between all languages or specific languages
   - For specific languages, enter comma-separated ISO codes (e.g., `en,fr,de,es`)

2. **Step 2**: Enter text to analyze
   - Type your own text or click a sample button to use pre-loaded examples

3. **Step 3**: Click "Detect Language" to identify the language

4. **Step 4**: (Optional) Get confidence score for a specific language
   - Enter a language code and click "Get Confidence"

5. **Step 5**: (Optional) View all confidence scores
   - Click "Get All Confidences" to see top 10 language matches with percentages

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

