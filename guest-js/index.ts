import { invoke } from '@tauri-apps/api/core'

export interface LanguageDetector {
  id: string;
}

export interface LanguageConfidence {
  language: string;
  confidence: number;
}

export async function ping(value: string): Promise<string | null> {
  return await invoke<{value?: string}>('plugin:lingua|ping', {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}

export async function createDetectorForAllLanguages(): Promise<LanguageDetector> {
  return await invoke<LanguageDetector>('plugin:lingua|create_detector_for_all_languages');
}

export async function createDetectorForLanguages(languages: string): Promise<LanguageDetector> {
  return await invoke<LanguageDetector>('plugin:lingua|create_detector_for_languages', {
    languages,
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
