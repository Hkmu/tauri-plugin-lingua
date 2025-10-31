use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::LinguaExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.lingua().ping(payload)
}

#[command]
pub(crate) async fn create_detector_for_all_languages<R: Runtime>(
    app: AppHandle<R>,
) -> Result<LanguageDetector> {
    app.lingua().create_detector_for_all_languages()
}

#[command]
pub(crate) async fn create_detector_for_languages<R: Runtime>(
    app: AppHandle<R>,
    languages: String,
) -> Result<LanguageDetector> {
    app.lingua().create_detector_for_languages(languages)
}

#[command]
pub(crate) async fn detect_language<R: Runtime>(
    app: AppHandle<R>,
    detector: LanguageDetector,
    text: String,
) -> Result<Option<String>> {
    app.lingua().detect_language(detector, text)
}

#[command]
pub(crate) async fn compute_language_confidence<R: Runtime>(
    app: AppHandle<R>,
    detector: LanguageDetector,
    text: String,
    language_code: String,
) -> Result<f64> {
    app.lingua().compute_language_confidence(detector, text, language_code)
}

#[command]
pub(crate) async fn compute_language_confidence_values<R: Runtime>(
    app: AppHandle<R>,
    detector: LanguageDetector,
    text: String,
) -> Result<Vec<LanguageConfidence>> {
    app.lingua().compute_language_confidence_values(detector, text)
}
