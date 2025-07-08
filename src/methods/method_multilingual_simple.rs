use super::method::EstimationMethod;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use whatlang::detect;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultilingualSimpleFeatures {
    pub char_count: usize,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultilingualSimpleParameters {
    pub coefficient: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultilingualSimpleMethodParameters {
    pub default_params: MultilingualSimpleParameters,
    pub language_params: HashMap<String, MultilingualSimpleParameters>,
}

impl Default for MultilingualSimpleParameters {
    fn default() -> Self {
        Self {
            coefficient: 0.329_268_3,
        }
    }
}

impl Default for MultilingualSimpleMethodParameters {
    fn default() -> Self {
        let mut language_params = HashMap::new();

        // ara
        language_params.insert(
            "ara".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.347_826_1,
            },
        );

        // ita
        language_params.insert(
            "ita".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.292_134_8,
            },
        );

        // fin
        language_params.insert(
            "fin".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.315_789_5,
            },
        );

        // nep
        language_params.insert(
            "nep".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.353_562,
            },
        );

        // ind
        language_params.insert(
            "ind".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.279_411_8,
            },
        );

        // fra
        language_params.insert(
            "fra".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.299_331_1,
            },
        );

        // nld
        language_params.insert(
            "nld".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.258_536_6,
            },
        );

        // eng
        language_params.insert(
            "eng".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.254_237_3,
            },
        );

        // tgl
        language_params.insert(
            "tgl".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.310_344_8,
            },
        );

        // ben
        language_params.insert(
            "ben".to_string(),
            MultilingualSimpleParameters { coefficient: 0.4 },
        );

        // sin
        language_params.insert(
            "sin".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.543_543_5,
            },
        );

        // sna
        language_params.insert(
            "sna".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.318_181_8,
            },
        );

        // jav
        language_params.insert(
            "jav".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.312_5,
            },
        );

        // est
        language_params.insert(
            "est".to_string(),
            MultilingualSimpleParameters { coefficient: 0.32 },
        );

        // guj
        language_params.insert(
            "guj".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.395_161_3,
            },
        );

        // por
        language_params.insert(
            "por".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.266_666_7,
            },
        );

        // lat
        language_params.insert(
            "lat".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.305_699_5,
            },
        );

        // lit
        language_params.insert(
            "lit".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.354_166_7,
            },
        );

        // tha
        language_params.insert(
            "tha".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.419_354_8,
            },
        );

        // vie
        language_params.insert(
            "vie".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.333_333_3,
            },
        );

        // hin
        language_params.insert(
            "hin".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.342_281_9,
            },
        );

        // tam
        language_params.insert(
            "tam".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.370_967_7,
            },
        );

        // slk
        language_params.insert(
            "slk".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.352_381,
            },
        );

        // rus
        language_params.insert(
            "rus".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.317_220_5,
            },
        );

        // mal
        language_params.insert(
            "mal".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.367_647,
            },
        );

        // khm
        language_params.insert(
            "khm".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.560_975_6,
            },
        );

        // mkd
        language_params.insert(
            "mkd".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.348_837_2,
            },
        );

        // jpn
        language_params.insert(
            "jpn".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.744_680_9,
            },
        );

        // dan
        language_params.insert(
            "dan".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.286_135_7,
            },
        );

        // yid
        language_params.insert(
            "yid".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.379_310_3,
            },
        );

        // afr
        language_params.insert(
            "afr".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.354_978_4,
            },
        );

        // epo
        language_params.insert(
            "epo".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.320_754_7,
            },
        );

        // cat
        language_params.insert(
            "cat".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.290_123_5,
            },
        );

        // slv
        language_params.insert(
            "slv".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.322_580_6,
            },
        );

        // ron
        language_params.insert(
            "ron".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.311_557_8,
            },
        );

        // spa
        language_params.insert(
            "spa".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.256_198_3,
            },
        );

        // kan
        language_params.insert(
            "kan".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.391_691_4,
            },
        );

        // bel
        language_params.insert(
            "bel".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.357_142_9,
            },
        );

        // kat
        language_params.insert(
            "kat".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.353_535_3,
            },
        );

        // heb
        language_params.insert(
            "heb".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.391_705_1,
            },
        );

        // hrv
        language_params.insert(
            "hrv".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.316_901_4,
            },
        );

        // mya
        language_params.insert(
            "mya".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.545_454_5,
            },
        );

        // tur
        language_params.insert(
            "tur".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.308_333_3,
            },
        );

        // cmn
        language_params.insert(
            "cmn".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.827_160_5,
            },
        );

        // amh
        language_params.insert(
            "amh".to_string(),
            MultilingualSimpleParameters {
                coefficient: 1.763_736,
            },
        );

        // srp
        language_params.insert(
            "srp".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.372_670_8,
            },
        );

        // ces
        language_params.insert(
            "ces".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.350_253_8,
            },
        );

        // nob
        language_params.insert(
            "nob".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.288_888_9,
            },
        );

        // pol
        language_params.insert(
            "pol".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.320_610_7,
            },
        );

        // pan
        language_params.insert(
            "pan".to_string(),
            MultilingualSimpleParameters { coefficient: 0.55 },
        );

        // mar
        language_params.insert(
            "mar".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.394_052,
            },
        );

        // deu
        language_params.insert(
            "deu".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.237_424_6,
            },
        );

        // tuk
        language_params.insert(
            "tuk".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.347_517_7,
            },
        );

        // pes
        language_params.insert(
            "pes".to_string(),
            MultilingualSimpleParameters { coefficient: 0.36 },
        );

        // tel
        language_params.insert(
            "tel".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.416_666_7,
            },
        );

        // uzb
        language_params.insert(
            "uzb".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.336_633_7,
            },
        );

        // zul
        language_params.insert(
            "zul".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.322_404_4,
            },
        );

        // ukr
        language_params.insert(
            "ukr".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.345_312_5,
            },
        );

        // kor
        language_params.insert(
            "kor".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.618_181_8,
            },
        );

        // bul
        language_params.insert(
            "bul".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.346_153_8,
            },
        );

        // aka
        language_params.insert(
            "aka".to_string(),
            MultilingualSimpleParameters { coefficient: 0.36 },
        );

        // hun
        language_params.insert(
            "hun".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.358_974_4,
            },
        );

        // lav
        language_params.insert(
            "lav".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.367_346_9,
            },
        );

        // swe
        language_params.insert(
            "swe".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.298_136_6,
            },
        );

        // ori
        language_params.insert(
            "ori".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.992_5,
            },
        );

        // urd
        language_params.insert(
            "urd".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.347_593_6,
            },
        );

        // ell
        language_params.insert(
            "ell".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.379_310_3,
            },
        );

        // hye
        language_params.insert(
            "hye".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.333_333_3,
            },
        );

        // aze
        language_params.insert(
            "aze".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.324_503_3,
            },
        );

        Self {
            default_params: MultilingualSimpleParameters::default(),
            language_params,
        }
    }
}

pub struct MultilingualSimpleMethod {
    parameters: MultilingualSimpleMethodParameters,
}

impl MultilingualSimpleMethod {
    pub fn new() -> Self {
        Self {
            parameters: MultilingualSimpleMethodParameters::default(),
        }
    }
}

impl Default for MultilingualSimpleMethod {
    fn default() -> Self {
        Self::new()
    }
}

impl EstimationMethod for MultilingualSimpleMethod {
    type Features = MultilingualSimpleFeatures;
    type Parameters = MultilingualSimpleMethodParameters;

    fn count(&self, text: &str) -> Self::Features {
        let char_count = text.chars().count();

        // Detect language
        let language = detect(text)
            .map(|info| info.lang().code())
            .unwrap_or("unknown")
            .to_string();

        MultilingualSimpleFeatures {
            char_count,
            language,
        }
    }

    fn estimate(&self, text: &str) -> usize {
        let features = self.count(text);

        // Handle empty text
        if features.char_count == 0 {
            return 0;
        }

        // Select parameters based on language
        let params = self
            .parameters
            .language_params
            .get(&features.language)
            .unwrap_or(&self.parameters.default_params);

        (features.char_count as f32 * params.coefficient).round() as usize
    }

    fn parameters(&self) -> Self::Parameters {
        self.parameters.clone()
    }

    fn set_parameters(&mut self, params: Self::Parameters) {
        self.parameters = params;
    }
}
