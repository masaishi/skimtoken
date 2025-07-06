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

        // Ara
        language_params.insert(
            "Ara".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3478260881025884,
            },
        );

        // Ita
        language_params.insert(
            "Ita".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.2921348311548165,
            },
        );

        // Fin
        language_params.insert(
            "Fin".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.31578947671545804,
            },
        );

        // Nep
        language_params.insert(
            "Nep".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.35356200463057796,
            },
        );

        // Ind
        language_params.insert(
            "Ind".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.2794117669100515,
            },
        );

        // Fra
        language_params.insert(
            "Fra".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.29933110199265395,
            },
        );

        // Nld
        language_params.insert(
            "Nld".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.2585365870617235,
            },
        );

        // Eng
        language_params.insert(
            "Eng".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.2542372931307417,
            },
        );

        // Tgl
        language_params.insert(
            "Tgl".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3103448252138147,
            },
        );

        // Ben
        language_params.insert(
            "Ben".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.399999998688826,
            },
        );

        // Sin
        language_params.insert(
            "Sin".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.5435435362784797,
            },
        );

        // Sna
        language_params.insert(
            "Sna".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3181818165263794,
            },
        );

        // Jav
        language_params.insert(
            "Jav".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.31249999555432356,
            },
        );

        // Est
        language_params.insert(
            "Est".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.32000000028635944,
            },
        );

        // Guj
        language_params.insert(
            "Guj".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3951612821613528,
            },
        );

        // Por
        language_params.insert(
            "Por".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.26666666434813796,
            },
        );

        // Lat
        language_params.insert(
            "Lat".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.30569947664440206,
            },
        );

        // Lit
        language_params.insert(
            "Lit".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.35416667555738257,
            },
        );

        // Tha
        language_params.insert(
            "Tha".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.41935484064361017,
            },
        );

        // Vie
        language_params.insert(
            "Vie".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.33333333346997535,
            },
        );

        // Hin
        language_params.insert(
            "Hin".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.342281884428011,
            },
        );

        // Tam
        language_params.insert(
            "Tam".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.37096773906181607,
            },
        );

        // Slk
        language_params.insert(
            "Slk".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3523809598212874,
            },
        );

        // Rus
        language_params.insert(
            "Rus".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.31722054375687125,
            },
        );

        // Mal
        language_params.insert(
            "Mal".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.36764704995286007,
            },
        );

        // Khm
        language_params.insert(
            "Khm".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.5609756177225682,
            },
        );

        // Mkd
        language_params.insert(
            "Mkd".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.34883720970986887,
            },
        );

        // Jpn
        language_params.insert(
            "Jpn".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.7446808590747476,
            },
        );

        // Dan
        language_params.insert(
            "Dan".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.2861356945460705,
            },
        );

        // Yid
        language_params.insert(
            "Yid".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3793103456013193,
            },
        );

        // Afr
        language_params.insert(
            "Afr".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3549783589889431,
            },
        );

        // Epo
        language_params.insert(
            "Epo".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.32075472552246104,
            },
        );

        // Cat
        language_params.insert(
            "Cat".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.2901234580742282,
            },
        );

        // Slv
        language_params.insert(
            "Slv".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.32258064644706685,
            },
        );

        // Ron
        language_params.insert(
            "Ron".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3115577912011593,
            },
        );

        // Spa
        language_params.insert(
            "Spa".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.25619834560831817,
            },
        );

        // Kan
        language_params.insert(
            "Kan".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.39169138704673045,
            },
        );

        // Bel
        language_params.insert(
            "Bel".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.35714286438515264,
            },
        );

        // Kat
        language_params.insert(
            "Kat".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.35353534973491235,
            },
        );

        // Heb
        language_params.insert(
            "Heb".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.39170506353200235,
            },
        );

        // Hrv
        language_params.insert(
            "Hrv".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3169014105934709,
            },
        );

        // Mya
        language_params.insert(
            "Mya".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.545454547771305,
            },
        );

        // Tur
        language_params.insert(
            "Tur".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.30833333166331983,
            },
        );

        // Cmn
        language_params.insert(
            "Cmn".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.8271604929439658,
            },
        );

        // Amh
        language_params.insert(
            "Amh".to_string(),
            MultilingualSimpleParameters {
                coefficient: 1.7637362770584277,
            },
        );

        // Srp
        language_params.insert(
            "Srp".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.37267080602549646,
            },
        );

        // Ces
        language_params.insert(
            "Ces".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.350253812410004,
            },
        );

        // Nob
        language_params.insert(
            "Nob".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.2888888903351371,
            },
        );

        // Pol
        language_params.insert(
            "Pol".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.32061068757150607,
            },
        );

        // Pan
        language_params.insert(
            "Pan".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.5500000010516604,
            },
        );

        // Mar
        language_params.insert(
            "Mar".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.39405204923261894,
            },
        );

        // Deu
        language_params.insert(
            "Deu".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.2374245520996768,
            },
        );

        // Tuk
        language_params.insert(
            "Tuk".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.34751773050525164,
            },
        );

        // Pes
        language_params.insert(
            "Pes".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.35999999440904334,
            },
        );

        // Tel
        language_params.insert(
            "Tel".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.416666662013471,
            },
        );

        // Uzb
        language_params.insert(
            "Uzb".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3366336601517637,
            },
        );

        // Zul
        language_params.insert(
            "Zul".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.32240437273228195,
            },
        );

        // Ukr
        language_params.insert(
            "Ukr".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3453125011371133,
            },
        );

        // Kor
        language_params.insert(
            "Kor".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.6181818176863859,
            },
        );

        // Bul
        language_params.insert(
            "Bul".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3461538407603729,
            },
        );

        // Aka
        language_params.insert(
            "Aka".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3599999951229915,
            },
        );

        // Hun
        language_params.insert(
            "Hun".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.35897435616504425,
            },
        );

        // Lav
        language_params.insert(
            "Lav".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3673469372197678,
            },
        );

        // Swe
        language_params.insert(
            "Swe".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.2981366410041522,
            },
        );

        // Ori
        language_params.insert(
            "Ori".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.9924999872204857,
            },
        );

        // Urd
        language_params.insert(
            "Urd".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3475935855530559,
            },
        );

        // Ell
        language_params.insert(
            "Ell".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3793103496681776,
            },
        );

        // Hye
        language_params.insert(
            "Hye".to_string(),
            MultilingualSimpleParameters {
                coefficient: 0.3333333323585104,
            },
        );

        // Aze
        language_params.insert(
            "Aze".to_string(),
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
