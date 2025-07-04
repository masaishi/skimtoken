#[cfg(feature = "pyo3")]
use pyo3::prelude::*;
use whatlang::Lang;

// =============================================================================
// CONFIGURATION PARAMETERS
// =============================================================================

// Weight parameters for different token types
const W_NUMBER: f64 = 0.45;
const W_SPECIAL: f64 = 0.6;
const W_EMOJI: f64 = 1.8;
const W_URL: f64 = 0.5;

// Language-specific weight configurations
const WEIGHTS_ENG: LanguageWeights = LanguageWeights {
    w_char: 0.09,
    w_word: 0.45,
    w_whitespace: -0.13,
    w_punctuation: 0.19,
};

const WEIGHTS_FRA: LanguageWeights = LanguageWeights {
    w_char: 0.11,
    w_word: 0.47,
    w_whitespace: -0.13,
    w_punctuation: 0.23,
};

const WEIGHTS_SPA: LanguageWeights = LanguageWeights {
    w_char: 0.1,
    w_word: 0.46,
    w_whitespace: -0.13,
    w_punctuation: 0.21,
};

const WEIGHTS_ITA: LanguageWeights = LanguageWeights {
    w_char: 0.1,
    w_word: 0.46,
    w_whitespace: -0.13,
    w_punctuation: 0.21,
};

const WEIGHTS_POR: LanguageWeights = LanguageWeights {
    w_char: 0.11,
    w_word: 0.47,
    w_whitespace: -0.13,
    w_punctuation: 0.22,
};

const WEIGHTS_CAT: LanguageWeights = LanguageWeights {
    w_char: 0.11,
    w_word: 0.46,
    w_whitespace: -0.13,
    w_punctuation: 0.21,
};

const WEIGHTS_NLD: LanguageWeights = LanguageWeights {
    w_char: 0.11,
    w_word: 0.43,
    w_whitespace: -0.13,
    w_punctuation: 0.21,
};

const WEIGHTS_DEU: LanguageWeights = LanguageWeights {
    w_char: 0.13,
    w_word: 0.42,
    w_whitespace: -0.13,
    w_punctuation: 0.23,
};

const WEIGHTS_AFR: LanguageWeights = LanguageWeights {
    w_char: 0.1,
    w_word: 0.45,
    w_whitespace: -0.13,
    w_punctuation: 0.19,
};

const WEIGHTS_ZUL: LanguageWeights = LanguageWeights {
    w_char: 0.12,
    w_word: 0.43,
    w_whitespace: -0.13,
    w_punctuation: 0.19,
};

const WEIGHTS_IND: LanguageWeights = LanguageWeights {
    w_char: 0.11,
    w_word: 0.45,
    w_whitespace: -0.13,
    w_punctuation: 0.19,
};

const WEIGHTS_TGL: LanguageWeights = LanguageWeights {
    w_char: 0.11,
    w_word: 0.44,
    w_whitespace: -0.13,
    w_punctuation: 0.19,
};

const WEIGHTS_TUR: LanguageWeights = LanguageWeights {
    w_char: 0.14,
    w_word: 0.42,
    w_whitespace: -0.13,
    w_punctuation: 0.21,
};

const WEIGHTS_CES: LanguageWeights = LanguageWeights {
    w_char: 0.13,
    w_word: 0.43,
    w_whitespace: -0.13,
    w_punctuation: 0.22,
};

const WEIGHTS_HUN: LanguageWeights = LanguageWeights {
    w_char: 0.15,
    w_word: 0.41,
    w_whitespace: -0.13,
    w_punctuation: 0.21,
};

const WEIGHTS_RUS: LanguageWeights = LanguageWeights {
    w_char: 0.13,
    w_word: 0.49,
    w_whitespace: -0.13,
    w_punctuation: 0.23,
};

const WEIGHTS_UKR: LanguageWeights = LanguageWeights {
    w_char: 0.14,
    w_word: 0.48,
    w_whitespace: -0.13,
    w_punctuation: 0.22,
};

const WEIGHTS_SRP: LanguageWeights = LanguageWeights {
    w_char: 0.13,
    w_word: 0.49,
    w_whitespace: -0.13,
    w_punctuation: 0.23,
};

const WEIGHTS_ARA: LanguageWeights = LanguageWeights {
    w_char: 0.31,
    w_word: 0.36,
    w_whitespace: -0.13,
    w_punctuation: 0.17,
};

const WEIGHTS_URD: LanguageWeights = LanguageWeights {
    w_char: 0.32,
    w_word: 0.35,
    w_whitespace: -0.13,
    w_punctuation: 0.26,
};

const WEIGHTS_HIN: LanguageWeights = LanguageWeights {
    w_char: 0.36,
    w_word: 0.31,
    w_whitespace: -0.13,
    w_punctuation: 0.21,
};

const WEIGHTS_BEN: LanguageWeights = LanguageWeights {
    w_char: 0.38,
    w_word: 0.3,
    w_whitespace: -0.13,
    w_punctuation: 0.22,
};

const WEIGHTS_MAR: LanguageWeights = LanguageWeights {
    w_char: 0.37,
    w_word: 0.31,
    w_whitespace: -0.13,
    w_punctuation: 0.21,
};

const WEIGHTS_NEP: LanguageWeights = LanguageWeights {
    w_char: 0.36,
    w_word: 0.32,
    w_whitespace: -0.13,
    w_punctuation: 0.22,
};

const WEIGHTS_TAM: LanguageWeights = LanguageWeights {
    w_char: 0.38,
    w_word: 0.29,
    w_whitespace: -0.13,
    w_punctuation: 0.24,
};

const WEIGHTS_TEL: LanguageWeights = LanguageWeights {
    w_char: 0.39,
    w_word: 0.28,
    w_whitespace: -0.13,
    w_punctuation: 0.24,
};

const WEIGHTS_VIE: LanguageWeights = LanguageWeights {
    w_char: 0.16,
    w_word: 0.4,
    w_whitespace: -0.13,
    w_punctuation: 0.19,
};

const WEIGHTS_THA: LanguageWeights = LanguageWeights {
    w_char: 0.4,
    w_word: 0.27,
    w_whitespace: -0.13,
    w_punctuation: 0.14,
};

const WEIGHTS_MYA: LanguageWeights = LanguageWeights {
    w_char: 0.43,
    w_word: 0.25,
    w_whitespace: -0.13,
    w_punctuation: 0.13,
};

const WEIGHTS_CJK: LanguageWeights = LanguageWeights {
    w_char: 0.0,
    w_word: 0.45,
    w_whitespace: -0.13,
    w_punctuation: 0.19,
};

const WEIGHTS_CODE: LanguageWeights = LanguageWeights {
    w_char: 0.1,
    w_word: 0.6,
    w_whitespace: -0.13,
    w_punctuation: 0.2,
};

const WEIGHTS_DEFAULT: LanguageWeights = LanguageWeights {
    w_char: 0.22,
    w_word: 0.38,
    w_whitespace: -0.13,
    w_punctuation: 0.19,
};

// CJK token costs
const CJK_CHINESE_COST: f64 = 1.0;
const CJK_KANA_COST: f64 = 1.0;
const CJK_KANA_BOUNDARY_COST: f64 = 1.5;
const CJK_KOREAN_COST: f64 = 1.1;
const CJK_OTHER_COST: f64 = 1.2;

// Adjustment factors
const LONG_WORD_MULTIPLIER: f64 = 1.05;
const LONG_WORD_THRESHOLD: f64 = 8.0;
const SHORT_TEXT_THRESHOLD: usize = 5;
const SHORT_TEXT_CHAR_MULTIPLIER: f64 = 0.75;
const WHITESPACE_ONLY_MULTIPLIER: f64 = 0.01;
const CJK_DOMINANCE_THRESHOLD: f64 = 0.3;
const CJK_OTHER_TOKEN_MULTIPLIER: f64 = 0.7;
const MIN_CODE_INDICATORS: usize = 3;

// =============================================================================
// DATA STRUCTURES
// =============================================================================

#[derive(Debug, Clone, Copy)]
pub struct LanguageWeights {
    w_char: f64,
    w_word: f64,
    w_whitespace: f64,
    w_punctuation: f64,
}

pub struct FeatureCounts {
    count_chars: usize,
    count_words: usize,
    count_numbers: usize,
    count_whitespace: usize,
    count_special_blocks: usize,
    count_cjk_chars: usize,
    count_punctuation: usize,
    count_emojis: usize,
    count_urls: usize,
    avg_word_length: f64,
}

// =============================================================================
// PROCESSING LOGIC
// =============================================================================

impl LanguageWeights {
    fn for_language(lang: Lang) -> Self {
        match lang {
            // Latin script languages
            Lang::Eng => WEIGHTS_ENG,
            Lang::Fra => WEIGHTS_FRA,
            Lang::Spa => WEIGHTS_SPA,
            Lang::Ita => WEIGHTS_ITA,
            Lang::Por => WEIGHTS_POR,
            Lang::Cat => WEIGHTS_CAT,
            Lang::Nld => WEIGHTS_NLD,
            Lang::Deu => WEIGHTS_DEU,
            Lang::Afr => WEIGHTS_AFR,
            Lang::Zul => WEIGHTS_ZUL,
            Lang::Ind => WEIGHTS_IND,
            Lang::Tgl => WEIGHTS_TGL,
            Lang::Tur => WEIGHTS_TUR,
            Lang::Ces => WEIGHTS_CES,
            Lang::Hun => WEIGHTS_HUN,

            // Cyrillic script languages
            Lang::Rus => WEIGHTS_RUS,
            Lang::Ukr => WEIGHTS_UKR,
            Lang::Srp => WEIGHTS_SRP,

            // Arabic script languages
            Lang::Ara => WEIGHTS_ARA,
            Lang::Urd => WEIGHTS_URD,

            // Indic scripts
            Lang::Hin => WEIGHTS_HIN,
            Lang::Ben => WEIGHTS_BEN,
            Lang::Mar => WEIGHTS_MAR,
            Lang::Nep => WEIGHTS_NEP,

            // South Indian scripts
            Lang::Tam => WEIGHTS_TAM,
            Lang::Tel => WEIGHTS_TEL,

            // Vietnamese (Latin with tones)
            Lang::Vie => WEIGHTS_VIE,

            // Thai script
            Lang::Tha => WEIGHTS_THA,

            // Burmese script
            Lang::Mya => WEIGHTS_MYA,

            // CJK languages
            Lang::Cmn | Lang::Jpn | Lang::Kor => WEIGHTS_CJK,

            // Default
            _ => WEIGHTS_DEFAULT,
        }
    }

    fn for_code() -> Self {
        WEIGHTS_CODE
    }
}

impl Default for LanguageWeights {
    fn default() -> Self {
        WEIGHTS_DEFAULT
    }
}

#[inline(always)]
fn is_cjk_char(ch: char) -> bool {
    matches!(ch as u32,
        0x4E00..=0x9FFF |   // CJK Unified Ideographs
        0x3400..=0x4DBF |   // CJK Extension A
        0x3040..=0x309F |   // Hiragana
        0x30A0..=0x30FF |   // Katakana
        0xAC00..=0xD7AF |   // Hangul
        0x3130..=0x318F |   // Hangul Compatibility Jamo
        0xFF00..=0xFFEF     // Halfwidth and Fullwidth Forms
    )
}

#[inline(always)]
fn is_emoji(ch: char) -> bool {
    // Simplified emoji detection - most common emoji ranges only
    matches!(ch as u32,
        0x1F300..=0x1F5FF | // Misc Symbols and Pictographs
        0x1F600..=0x1F64F | // Emoticons
        0x1F680..=0x1F6FF   // Transport and Map
    )
}

fn get_cjk_token_count(text: &str) -> f64 {
    let mut count = 0.0;
    let chars: Vec<char> = text.chars().collect();

    for i in 0..chars.len() {
        let ch = chars[i];
        if is_cjk_char(ch) {
            if matches!(ch as u32, 0x4E00..=0x9FFF) {
                // Common Chinese characters
                count += CJK_CHINESE_COST;
            } else if matches!(ch as u32, 0x3040..=0x30FF) {
                // Hiragana/Katakana
                if i + 1 < chars.len() && (chars[i + 1].is_ascii() || chars[i + 1].is_whitespace())
                {
                    count += CJK_KANA_BOUNDARY_COST;
                } else {
                    count += CJK_KANA_COST;
                }
            } else if matches!(ch as u32, 0xAC00..=0xD7AF) {
                // Korean syllabic blocks
                count += CJK_KOREAN_COST;
            } else {
                count += CJK_OTHER_COST;
            }
        }
    }

    count
}

fn extract_features(text: &str) -> FeatureCounts {
    let mut counts = FeatureCounts {
        count_chars: 0,
        count_words: 0,
        count_numbers: 0,
        count_whitespace: 0,
        count_special_blocks: 0,
        count_cjk_chars: 0,
        count_punctuation: 0,
        count_emojis: 0,
        count_urls: 0,
        avg_word_length: 0.0,
    };

    let mut in_word = false;
    let mut in_number = false;
    let mut in_special = false;
    let mut word_lengths: Vec<usize> = Vec::with_capacity(32);
    let mut current_word_len = 0;

    // Check for URLs
    if text.contains("http://") || text.contains("https://") || text.contains("www.") {
        counts.count_urls = text.matches("http").count() + text.matches("www.").count();
    }

    for ch in text.chars() {
        counts.count_chars += 1;

        if is_cjk_char(ch) {
            counts.count_cjk_chars += 1;
            in_word = false;
            in_number = false;
            in_special = false;
            if current_word_len > 0 {
                word_lengths.push(current_word_len);
                current_word_len = 0;
            }
        } else if is_emoji(ch) {
            counts.count_emojis += 1;
            in_word = false;
            in_number = false;
            in_special = false;
        } else if ch.is_whitespace() {
            counts.count_whitespace += 1;
            in_word = false;
            in_number = false;
            in_special = false;
            if current_word_len > 0 {
                word_lengths.push(current_word_len);
                current_word_len = 0;
            }
        } else if ch.is_ascii_digit() {
            if !in_number {
                counts.count_numbers += 1;
                in_number = true;
            }
            in_word = false;
            in_special = false;
        } else if ch.is_alphabetic() {
            if !in_word {
                counts.count_words += 1;
                in_word = true;
            }
            current_word_len += 1;
            in_number = false;
            in_special = false;
        } else if matches!(ch, '.' | ',' | '!' | '?' | ':' | ';' | '-' | '\'' | '"') {
            counts.count_punctuation += 1;
            in_word = false;
            in_number = false;
            in_special = false;
            if current_word_len > 0 {
                word_lengths.push(current_word_len);
                current_word_len = 0;
            }
        } else {
            if !in_special {
                counts.count_special_blocks += 1;
                in_special = true;
            }
            in_word = false;
            in_number = false;
        }
    }

    if current_word_len > 0 {
        word_lengths.push(current_word_len);
    }

    if !word_lengths.is_empty() {
        counts.avg_word_length =
            word_lengths.iter().sum::<usize>() as f64 / word_lengths.len() as f64;
    }

    counts
}

fn is_likely_code(text: &str) -> bool {
    // Quick check for common code patterns without lowercase conversion
    let mut indicator_count = 0;

    // Check for most common indicators first
    if text.contains("{") || text.contains("}") || text.contains(";") {
        indicator_count += 1;
    }
    if text.contains("function") || text.contains("def ") || text.contains("class ") {
        indicator_count += 1;
    }
    if text.contains("return") || text.contains("if (") || text.contains("for (") {
        indicator_count += 1;
    }

    indicator_count >= MIN_CODE_INDICATORS
}

pub fn estimate_tokens_internal(text: &str) -> u32 {
    if text.is_empty() {
        return 0;
    }

    // Handle pure whitespace
    if text.trim().is_empty() {
        return (text.len() as f64 * WHITESPACE_ONLY_MULTIPLIER)
            .round()
            .max(1.0) as u32;
    }

    let features = extract_features(text);

    // Special handling for CJK text
    if features.count_cjk_chars > 0 {
        let cjk_ratio = features.count_cjk_chars as f64 / features.count_chars as f64;

        if cjk_ratio > CJK_DOMINANCE_THRESHOLD {
            let cjk_tokens = get_cjk_token_count(text);
            let other_tokens = features.count_words as f64 * CJK_OTHER_TOKEN_MULTIPLIER
                + features.count_numbers as f64 * W_NUMBER
                + features.count_emojis as f64 * W_EMOJI
                + features.count_punctuation as f64 * 0.2;
            return (cjk_tokens + other_tokens).round().max(1.0) as u32;
        }
    }

    // Skip language detection for texts with significant CJK content
    let weights = if features.count_cjk_chars > features.count_chars / 4 {
        WEIGHTS_CJK
    } else if is_likely_code(text) {
        LanguageWeights::for_code()
    } else {
        // Detect language and use appropriate weights
        match whatlang::detect(text) {
            Some(info) => LanguageWeights::for_language(info.lang()),
            None => WEIGHTS_DEFAULT,
        }
    };

    // Base estimate
    let mut estimate = weights.w_char * features.count_chars as f64
        + weights.w_word * features.count_words as f64
        + W_NUMBER * features.count_numbers as f64
        + weights.w_whitespace * features.count_whitespace as f64
        + W_SPECIAL * features.count_special_blocks as f64
        + weights.w_punctuation * features.count_punctuation as f64
        + W_EMOJI * features.count_emojis as f64
        + W_URL * features.count_urls as f64;

    // Adjust for word length
    if features.avg_word_length > LONG_WORD_THRESHOLD {
        estimate *= LONG_WORD_MULTIPLIER;
    }

    // Handle very short text
    if features.count_chars < SHORT_TEXT_THRESHOLD {
        estimate = estimate.max(features.count_chars as f64 * SHORT_TEXT_CHAR_MULTIPLIER);
    }

    estimate.max(1.0).round() as u32
}

#[cfg(feature = "pyo3")]
#[pyfunction]
fn estimate_tokens(text: &str) -> u32 {
    estimate_tokens_internal(text)
}

#[cfg(feature = "pyo3")]
#[pymodule]
fn _skimtoken_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(estimate_tokens, m)?)?;
    Ok(())
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(estimate_tokens_internal(""), 0);
    }

    #[test]
    fn test_simple_english() {
        let text = "This is a simple test.";
        let estimate = estimate_tokens_internal(text);
        assert!(estimate > 0);
        assert!(estimate < 20);
    }

    #[test]
    fn test_code_detection() {
        let code = "def hello_world():\n    print('Hello, World!')";
        let estimate = estimate_tokens_internal(code);
        assert!(estimate > 0);
    }

    #[test]
    fn test_chinese_text() {
        let text = "这是一个简单的测试。";
        let estimate = estimate_tokens_internal(text);
        assert!(estimate > 0);
    }

    #[test]
    fn test_mixed_content() {
        let text = "The price is $42.99 for 3 items!";
        let estimate = estimate_tokens_internal(text);
        assert!(estimate > 0);
    }

    #[test]
    fn test_cjk_detection() {
        assert!(is_cjk_char('中'));
        assert!(is_cjk_char('あ'));
        assert!(is_cjk_char('ア'));
        assert!(is_cjk_char('한'));
        assert!(!is_cjk_char('A'));
        assert!(!is_cjk_char('1'));
    }
}
