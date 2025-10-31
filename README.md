# Tauri Plugin Lingua

A Tauri plugin for language detection using [lingua-rs](https://github.com/pemistahl/lingua-rs).

## Features

- Detect language from text with high accuracy
- Support for 75 languages
- Confidence scoring for language detection
- Works with both long and short text fragments
- Thread-safe detector instances
- Works on both desktop and mobile platforms

## Installation

### Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
tauri-plugin-lingua = "0.1.0"
```

### JavaScript

Install using pnpm:

```bash
pnpm add tauri-plugin-lingua-api
```

## Usage

### Initialize Plugin

In your Tauri app's `main.rs` or `lib.rs`:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_lingua::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### TypeScript API

```typescript
import {
  createDetectorForAllLanguages,
  createDetectorForLanguages,
  detectLanguage,
  computeLanguageConfidence,
  computeLanguageConfidenceValues,
  type LanguageDetector,
  type LanguageConfidence,
} from 'tauri-plugin-lingua-api';

// Create a detector for all 75 supported languages
const detector = await createDetectorForAllLanguages();

// Or create a detector for specific languages (comma-separated ISO 639-1 codes)
const customDetector = await createDetectorForLanguages('en,fr,de,es,zh');

// Detect the language of a text
const language = await detectLanguage(detector, "Hello, how are you?");
console.log(language); // "EN"

// Detect language with null handling
const lang = await detectLanguage(detector, "Hi");
if (lang) {
  console.log(`Detected language: ${lang}`);
}

// Get confidence score for a specific language
const confidence = await computeLanguageConfidence(
  detector,
  "Bonjour le monde",
  "FR"
);
console.log(confidence); // e.g., 0.95

// Get confidence values for all languages
const confidences = await computeLanguageConfidenceValues(
  detector,
  "Hola mundo"
);
// Returns: [
//   { language: "ES", confidence: 0.98 },
//   { language: "PT", confidence: 0.45 },
//   ...
// ]
```

## API Reference

### Types

#### `LanguageDetector`

```typescript
interface LanguageDetector {
  id: string;
}
```

#### `LanguageConfidence`

```typescript
interface LanguageConfidence {
  language: string;  // ISO 639-1 code (e.g., "EN", "FR")
  confidence: number; // 0.0 to 1.0
}
```

### Functions

#### `createDetectorForAllLanguages()`

Creates a language detector for all 75 supported languages.

**Returns:** `Promise<LanguageDetector>`

#### `createDetectorForLanguages(languages: string)`

Creates a language detector for specific languages.

**Parameters:**
- `languages`: Comma-separated ISO 639-1 language codes (e.g., `"en,fr,de"`)

**Returns:** `Promise<LanguageDetector>`

**Throws:** Error if invalid language codes are provided

#### `detectLanguage(detector: LanguageDetector, text: string)`

Detects the language of the given text.

**Parameters:**
- `detector`: A language detector instance
- `text`: The text to analyze

**Returns:** `Promise<string | null>` - ISO 639-1 code or null if language cannot be detected

#### `computeLanguageConfidence(detector: LanguageDetector, text: string, languageCode: string)`

Computes the confidence score for a specific language.

**Parameters:**
- `detector`: A language detector instance
- `text`: The text to analyze
- `languageCode`: ISO 639-1 code of the language to check

**Returns:** `Promise<number>` - Confidence score between 0.0 and 1.0

#### `computeLanguageConfidenceValues(detector: LanguageDetector, text: string)`

Computes confidence scores for all languages.

**Parameters:**
- `detector`: A language detector instance
- `text`: The text to analyze

**Returns:** `Promise<LanguageConfidence[]>` - Array of language-confidence pairs, sorted by confidence (highest first)

## Supported Languages

The plugin supports 75 languages via ISO 639-1 codes:

AF (Afrikaans), SQ (Albanian), AR (Arabic), HY (Armenian), AZ (Azerbaijani), EU (Basque), BE (Belarusian), BN (Bengali), NB (Norwegian Bokmal), BS (Bosnian), BG (Bulgarian), CA (Catalan), ZH (Chinese), HR (Croatian), CS (Czech), DA (Danish), NL (Dutch), EN (English), EO (Esperanto), ET (Estonian), FI (Finnish), FR (French), LG (Ganda), KA (Georgian), DE (German), EL (Greek), GU (Gujarati), HE (Hebrew), HI (Hindi), HU (Hungarian), IS (Icelandic), ID (Indonesian), GA (Irish), IT (Italian), JA (Japanese), KK (Kazakh), KO (Korean), LA (Latin), LV (Latvian), LT (Lithuanian), MK (Macedonian), MS (Malay), MI (Maori), MR (Marathi), MN (Mongolian), NN (Norwegian Nynorsk), FA (Persian), PL (Polish), PT (Portuguese), PA (Punjabi), RO (Romanian), RU (Russian), SR (Serbian), SN (Shona), SK (Slovak), SL (Slovene), SO (Somali), ST (Sotho), ES (Spanish), SW (Swahili), SV (Swedish), TL (Tagalog), TA (Tamil), TE (Telugu), TH (Thai), TS (Tsonga), TN (Tswana), TR (Turkish), UK (Ukrainian), UR (Urdu), VI (Vietnamese), CY (Welsh), XH (Xhosa), YO (Yoruba), ZU (Zulu)

## License

Same as [lingua-rs](https://github.com/pemistahl/lingua-rs) - Apache 2.0
