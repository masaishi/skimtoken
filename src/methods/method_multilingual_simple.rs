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
    pub coefficient: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultilingualSimpleMethodParameters {
    pub default_params: MultilingualSimpleParameters,
    pub language_params: HashMap<String, MultilingualSimpleParameters>,
}

impl Default for MultilingualSimpleParameters {
    fn default() -> Self {
        Self {
            coefficient: 0.32926829331569196,
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
                coefficient: 0.3478260881025884,
            },
        );

        // ita
        language_params.insert(
            "ita".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.2921348311548165,
            },
        );

        // fin
        language_params.insert(
            "fin".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.31578947671545804,
            },
        );

        // nep
        language_params.insert(
            "nep".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.35356200463057796,
            },
        );

        // ind
        language_params.insert(
            "ind".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.2794117669100515,
            },
        );

        // fra
        language_params.insert(
            "fra".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.29933110199265395,
            },
        );

        // nld
        language_params.insert(
            "nld".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.2585365870617235,
            },
        );

        // eng
        language_params.insert(
            "eng".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.2542372931307417,
            },
        );

        // tgl
        language_params.insert(
            "tgl".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3103448252138147,
            },
        );

        // ben
        language_params.insert(
            "ben".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.399999998688826,
            },
        );

        // sin
        language_params.insert(
            "sin".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.5435435362784797,
            },
        );

        // sna
        language_params.insert(
            "sna".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3181818165263794,
            },
        );

        // jav
        language_params.insert(
            "jav".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.31249999555432356,
            },
        );

        // est
        language_params.insert(
            "est".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.32000000028635944,
            },
        );

        // guj
        language_params.insert(
            "guj".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3951612821613528,
            },
        );

        // por
        language_params.insert(
            "por".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.26666666434813796,
            },
        );

        // lat
        language_params.insert(
            "lat".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.30569947664440206,
            },
        );

        // lit
        language_params.insert(
            "lit".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.35416667555738257,
            },
        );

        // tha
        language_params.insert(
            "tha".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.41935484064361017,
            },
        );

        // vie
        language_params.insert(
            "vie".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.33333333346997535,
            },
        );

        // hin
        language_params.insert(
            "hin".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.342281884428011,
            },
        );

        // tam
        language_params.insert(
            "tam".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.37096773906181607,
            },
        );

        // slk
        language_params.insert(
            "slk".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3523809598212874,
            },
        );

        // rus
        language_params.insert(
            "rus".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.31722054375687125,
            },
        );

        // mal
        language_params.insert(
            "mal".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.36764704995286007,
            },
        );

        // khm
        language_params.insert(
            "khm".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.5609756177225682,
            },
        );

        // mkd
        language_params.insert(
            "mkd".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.34883720970986887,
            },
        );

        // jpn
        language_params.insert(
            "jpn".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.7446808590747476,
            },
        );

        // dan
        language_params.insert(
            "dan".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.2861356945460705,
            },
        );

        // yid
        language_params.insert(
            "yid".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3793103456013193,
            },
        );

        // afr
        language_params.insert(
            "afr".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3549783589889431,
            },
        );

        // epo
        language_params.insert(
            "epo".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.32075472552246104,
            },
        );

        // cat
        language_params.insert(
            "cat".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.2901234580742282,
            },
        );

        // slv
        language_params.insert(
            "slv".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.32258064644706685,
            },
        );

        // ron
        language_params.insert(
            "ron".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3115577912011593,
            },
        );

        // spa
        language_params.insert(
            "spa".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.25619834560831817,
            },
        );

        // kan
        language_params.insert(
            "kan".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.39169138704673045,
            },
        );

        // bel
        language_params.insert(
            "bel".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.35714286438515264,
            },
        );

        // kat
        language_params.insert(
            "kat".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.35353534973491235,
            },
        );

        // heb
        language_params.insert(
            "heb".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.39170506353200235,
            },
        );

        // hrv
        language_params.insert(
            "hrv".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3169014105934709,
            },
        );

        // mya
        language_params.insert(
            "mya".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.545454547771305,
            },
        );

        // tur
        language_params.insert(
            "tur".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.30833333166331983,
            },
        );

        // cmn
        language_params.insert(
            "cmn".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.8271604929439658,
            },
        );

        // amh
        language_params.insert(
            "amh".to_string(),
            MultilingualSimpleParameters {
                coefficient: 1.7637362770584277,
            },
        );

        // srp
        language_params.insert(
            "srp".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.37267080602549646,
            },
        );

        // ces
        language_params.insert(
            "ces".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.350253812410004,
            },
        );

        // nob
        language_params.insert(
            "nob".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.2888888903351371,
            },
        );

        // pol
        language_params.insert(
            "pol".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.32061068757150607,
            },
        );

        // pan
        language_params.insert(
            "pan".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.5500000010516604,
            },
        );

        // mar
        language_params.insert(
            "mar".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.39405204923261894,
            },
        );

        // deu
        language_params.insert(
            "deu".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.2374245520996768,
            },
        );

        // tuk
        language_params.insert(
            "tuk".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.34751773050525164,
            },
        );

        // pes
        language_params.insert(
            "pes".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.35999999440904334,
            },
        );

        // tel
        language_params.insert(
            "tel".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.416666662013471,
            },
        );

        // uzb
        language_params.insert(
            "uzb".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3366336601517637,
            },
        );

        // zul
        language_params.insert(
            "zul".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.32240437273228195,
            },
        );

        // ukr
        language_params.insert(
            "ukr".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3453125011371133,
            },
        );

        // kor
        language_params.insert(
            "kor".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.6181818176863859,
            },
        );

        // bul
        language_params.insert(
            "bul".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3461538407603729,
            },
        );

        // aka
        language_params.insert(
            "aka".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3599999951229915,
            },
        );

        // hun
        language_params.insert(
            "hun".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.35897435616504425,
            },
        );

        // lav
        language_params.insert(
            "lav".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3673469372197678,
            },
        );

        // swe
        language_params.insert(
            "swe".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.2981366410041522,
            },
        );

        // ori
        language_params.insert(
            "ori".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.9924999872204857,
            },
        );

        // urd
        language_params.insert(
            "urd".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3475935855530559,
            },
        );

        // ell
        language_params.insert(
            "ell".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3793103496681776,
            },
        );

        // hye
        language_params.insert(
            "hye".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3333333323585104,
            },
        );

        // aze
        language_params.insert(
            "aze".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.32450331123755793,
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

        (features.char_count as f64 * params.coefficient).round() as usize
    }

    fn parameters(&self) -> Self::Parameters {
        self.parameters.clone()
    }

    fn set_parameters(&mut self, params: Self::Parameters) {
        self.parameters = params;
    }
}
