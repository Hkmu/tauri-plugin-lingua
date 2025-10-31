use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use lingua::{Language, LanguageDetectorBuilder};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Lingua<R>> {
  Ok(Lingua {
    app: app.clone(),
    detectors: Arc::new(RwLock::new(HashMap::new())),
    next_id: Arc::new(RwLock::new(0)),
  })
}

/// Access to the lingua APIs.
pub struct Lingua<R: Runtime> {
  #[allow(dead_code)]
  app: AppHandle<R>,
  detectors: Arc<RwLock<HashMap<String, lingua::LanguageDetector>>>,
  next_id: Arc<RwLock<u64>>,
}

impl<R: Runtime> Lingua<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }

  pub fn create_detector_for_all_languages(&self) -> crate::Result<LanguageDetector> {
    let detector = LanguageDetectorBuilder::from_all_languages().build();
    let mut next_id = self.next_id.write();
    let id = next_id.to_string();
    *next_id += 1;
    
    self.detectors.write().insert(id.clone(), detector);
    
    Ok(LanguageDetector { id })
  }

  pub fn create_detector_for_languages(&self, languages: String) -> crate::Result<LanguageDetector> {
    let language_codes: Vec<&str> = languages.split(',').collect();
    let mut selected_languages = Vec::new();
    
    for code in language_codes {
      let code = code.trim().to_uppercase();
      if let Some(language) = parse_language_code(&code) {
        selected_languages.push(language);
      } else {
        return Err(crate::Error::InvalidLanguageCode(code));
      }
    }
    
    if selected_languages.is_empty() {
      return Err(crate::Error::NoLanguagesProvided);
    }
    
    let detector = LanguageDetectorBuilder::from_languages(&selected_languages).build();
    let mut next_id = self.next_id.write();
    let id = next_id.to_string();
    *next_id += 1;
    
    self.detectors.write().insert(id.clone(), detector);
    
    Ok(LanguageDetector { id })
  }

  pub fn detect_language(&self, detector_ref: LanguageDetector, text: String) -> crate::Result<Option<String>> {
    let detectors = self.detectors.read();
    let detector = detectors.get(&detector_ref.id)
      .ok_or_else(|| crate::Error::DetectorNotFound)?;
    
    Ok(detector.detect_language_of(&text).map(|lang| lang.iso_code_639_1().to_string()))
  }

  pub fn compute_language_confidence(
    &self,
    detector_ref: LanguageDetector,
    text: String,
    language_code: String,
  ) -> crate::Result<f64> {
    let detectors = self.detectors.read();
    let detector = detectors.get(&detector_ref.id)
      .ok_or_else(|| crate::Error::DetectorNotFound)?;
    
    let code = language_code.to_uppercase();
    let language = parse_language_code(&code)
      .ok_or_else(|| crate::Error::InvalidLanguageCode(language_code))?;
    
    Ok(detector.compute_language_confidence(&text, language))
  }

  pub fn compute_language_confidence_values(
    &self,
    detector_ref: LanguageDetector,
    text: String,
  ) -> crate::Result<Vec<LanguageConfidence>> {
    let detectors = self.detectors.read();
    let detector = detectors.get(&detector_ref.id)
      .ok_or_else(|| crate::Error::DetectorNotFound)?;
    
    let values = detector.compute_language_confidence_values(&text);
    
    Ok(values.into_iter().map(|(lang, conf)| LanguageConfidence {
      language: lang.iso_code_639_1().to_string(),
      confidence: conf,
    }).collect())
  }
}

fn parse_language_code(code: &str) -> Option<Language> {
  match code {
    "AF" => Some(Language::Afrikaans),
    "SQ" => Some(Language::Albanian),
    "AR" => Some(Language::Arabic),
    "HY" => Some(Language::Armenian),
    "AZ" => Some(Language::Azerbaijani),
    "EU" => Some(Language::Basque),
    "BE" => Some(Language::Belarusian),
    "BN" => Some(Language::Bengali),
    "NB" => Some(Language::Bokmal),
    "BS" => Some(Language::Bosnian),
    "BG" => Some(Language::Bulgarian),
    "CA" => Some(Language::Catalan),
    "ZH" => Some(Language::Chinese),
    "HR" => Some(Language::Croatian),
    "CS" => Some(Language::Czech),
    "DA" => Some(Language::Danish),
    "NL" => Some(Language::Dutch),
    "EN" => Some(Language::English),
    "EO" => Some(Language::Esperanto),
    "ET" => Some(Language::Estonian),
    "FI" => Some(Language::Finnish),
    "FR" => Some(Language::French),
    "LG" => Some(Language::Ganda),
    "KA" => Some(Language::Georgian),
    "DE" => Some(Language::German),
    "EL" => Some(Language::Greek),
    "GU" => Some(Language::Gujarati),
    "HE" => Some(Language::Hebrew),
    "HI" => Some(Language::Hindi),
    "HU" => Some(Language::Hungarian),
    "IS" => Some(Language::Icelandic),
    "ID" => Some(Language::Indonesian),
    "GA" => Some(Language::Irish),
    "IT" => Some(Language::Italian),
    "JA" => Some(Language::Japanese),
    "KK" => Some(Language::Kazakh),
    "KO" => Some(Language::Korean),
    "LA" => Some(Language::Latin),
    "LV" => Some(Language::Latvian),
    "LT" => Some(Language::Lithuanian),
    "MK" => Some(Language::Macedonian),
    "MS" => Some(Language::Malay),
    "MI" => Some(Language::Maori),
    "MR" => Some(Language::Marathi),
    "MN" => Some(Language::Mongolian),
    "NN" => Some(Language::Nynorsk),
    "FA" => Some(Language::Persian),
    "PL" => Some(Language::Polish),
    "PT" => Some(Language::Portuguese),
    "PA" => Some(Language::Punjabi),
    "RO" => Some(Language::Romanian),
    "RU" => Some(Language::Russian),
    "SR" => Some(Language::Serbian),
    "SN" => Some(Language::Shona),
    "SK" => Some(Language::Slovak),
    "SL" => Some(Language::Slovene),
    "SO" => Some(Language::Somali),
    "ST" => Some(Language::Sotho),
    "ES" => Some(Language::Spanish),
    "SW" => Some(Language::Swahili),
    "SV" => Some(Language::Swedish),
    "TL" => Some(Language::Tagalog),
    "TA" => Some(Language::Tamil),
    "TE" => Some(Language::Telugu),
    "TH" => Some(Language::Thai),
    "TS" => Some(Language::Tsonga),
    "TN" => Some(Language::Tswana),
    "TR" => Some(Language::Turkish),
    "UK" => Some(Language::Ukrainian),
    "UR" => Some(Language::Urdu),
    "VI" => Some(Language::Vietnamese),
    "CY" => Some(Language::Welsh),
    "XH" => Some(Language::Xhosa),
    "YO" => Some(Language::Yoruba),
    "ZU" => Some(Language::Zulu),
    _ => None,
  }
}
