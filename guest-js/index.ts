import { invoke } from '@tauri-apps/api/core'

export interface LanguageDetector {
  id: string;
}

export interface LanguageConfidence {
  language: string;
  confidence: number;
}

export interface CreateDetectorOptions {
  /** Minimum relative distance between confidence values.
   *  Default: -1 (disabled)
   *  Range: -1 or 0.0-0.99
   *  When set to a value between 0.0-0.99, language detection returns null if
   *  the difference between the top two language confidences is smaller than
   *  this value. Set to -1 to disable this filtering. */
  minimumRelativeDistance?: number;
}

export async function ping(value: string): Promise<string | null> {
  return await invoke<{value?: string}>('plugin:lingua|ping', {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}

export async function createDetectorForAllLanguages(options?: CreateDetectorOptions): Promise<LanguageDetector> {
  return await invoke<LanguageDetector>('plugin:lingua|create_detector_for_all_languages', {
    options: options ?? {},
  });
}

export async function createDetectorForLanguages(languages: string, options?: CreateDetectorOptions): Promise<LanguageDetector> {
  return await invoke<LanguageDetector>('plugin:lingua|create_detector_for_languages', {
    languages,
    options: options ?? {},
  });
}

export async function detectLanguage(detector: LanguageDetector, text: string): Promise<string | null> {
  return await invoke<string | null>('plugin:lingua|detect_language', {
    detector,
    text,
  });
}

export async function computeLanguageConfidence(
  detector: LanguageDetector,
  text: string,
  languageCode: string
): Promise<number> {
  return await invoke<number>('plugin:lingua|compute_language_confidence', {
    detector,
    text,
    languageCode,
  });
}

export async function computeLanguageConfidenceValues(
  detector: LanguageDetector,
  text: string
): Promise<LanguageConfidence[]> {
  return await invoke<LanguageConfidence[]>('plugin:lingua|compute_language_confidence_values', {
    detector,
    text,
  });
}
