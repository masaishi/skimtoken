use super::method::EstimationMethod;
use super::method_basic::BasicFeatures;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use whatlang::detect;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultilingualFeatures {
    pub basic_features: BasicFeatures,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultilingualParameters {
    pub char_coef: f32,
    pub word_coef: f32,
    pub avg_word_length_coef: f32,
    pub space_coef: f32,
    pub intercept: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultilingualMethodParameters {
    pub default_params: MultilingualParameters,
    pub language_params: HashMap<String, MultilingualParameters>,
}

impl Default for MultilingualParameters {
    fn default() -> Self {
        Self {
            char_coef: 0.321_774_5,
            word_coef: 0.070_228_82,
            avg_word_length_coef: 0.509_098_2,
            space_coef: -0.158_310_9,
            intercept: 1.591_021,
        }
    }
}

impl Default for MultilingualMethodParameters {
    fn default() -> Self {
        let mut language_params = HashMap::new();

        // ara
        language_params.insert(
            "ara".to_string(),
            MultilingualParameters {
                char_coef: 0.448_802_4,
                word_coef: -18.384_45,
                avg_word_length_coef: 1.797_304,
                space_coef: 17.888_36,
                intercept: 10.365_54,
            },
        );

        // ita
        language_params.insert(
            "ita".to_string(),
            MultilingualParameters {
                char_coef: 0.126_874_2,
                word_coef: 0.792_041_4,
                avg_word_length_coef: 0.979_486_5,
                space_coef: 0.183_067,
                intercept: -2.993_577,
            },
        );

        // fin
        language_params.insert(
            "fin".to_string(),
            MultilingualParameters {
                char_coef: 0.297_603,
                word_coef: 0.110_460_5,
                avg_word_length_coef: 0.161_561_5,
                space_coef: -0.143_074_2,
                intercept: 1.465_598,
            },
        );

        // nep
        language_params.insert(
            "nep".to_string(),
            MultilingualParameters {
                char_coef: 0.342_030_1,
                word_coef: -5.602_272,
                avg_word_length_coef: 0.402_932_3,
                space_coef: 5.458_267,
                intercept: 7.532_691,
            },
        );

        // ind
        language_params.insert(
            "ind".to_string(),
            MultilingualParameters {
                char_coef: 0.266_979_4,
                word_coef: -0.008_999_044,
                avg_word_length_coef: 0.155_462_6,
                space_coef: -0.074_574_81,
                intercept: 2.074_445,
            },
        );

        // fra
        language_params.insert(
            "fra".to_string(),
            MultilingualParameters {
                char_coef: 0.181_814_1,
                word_coef: 0.548_359_6,
                avg_word_length_coef: 0.620_022,
                space_coef: 0.084_003_84,
                intercept: -1.556_145,
            },
        );

        // nld
        language_params.insert(
            "nld".to_string(),
            MultilingualParameters {
                char_coef: 0.130_532_2,
                word_coef: 0.392_070_9,
                avg_word_length_coef: 1.232_021,
                space_coef: 0.425_733_8,
                intercept: -4.518_895,
            },
        );

        // eng
        language_params.insert(
            "eng".to_string(),
            MultilingualParameters {
                char_coef: 0.347_931_3,
                word_coef: -0.550_550_4,
                avg_word_length_coef: 0.079_289_83,
                space_coef: -0.043_495_66,
                intercept: 2.853_936,
            },
        );

        // tgl
        language_params.insert(
            "tgl".to_string(),
            MultilingualParameters {
                char_coef: 0.368_151_5,
                word_coef: -0.500_993_6,
                avg_word_length_coef: 0.117_350_7,
                space_coef: 0.007_593_871,
                intercept: 1.755_873,
            },
        );

        // ben
        language_params.insert(
            "ben".to_string(),
            MultilingualParameters {
                char_coef: 0.381_057_7,
                word_coef: 0.015_434_32,
                avg_word_length_coef: 0.075_770_55,
                space_coef: 0.015_434_32,
                intercept: 1.000_29,
            },
        );

        // sin
        language_params.insert(
            "sin".to_string(),
            MultilingualParameters {
                char_coef: 0.521_049_5,
                word_coef: 0.690_805_1,
                avg_word_length_coef: 0.248_014_4,
                space_coef: -0.594_142_2,
                intercept: -1.245_287,
            },
        );

        // sna
        language_params.insert(
            "sna".to_string(),
            MultilingualParameters {
                char_coef: 0.309_188_2,
                word_coef: -0.037_233_15,
                avg_word_length_coef: 0.268_528_6,
                space_coef: -0.130_344_4,
                intercept: 1.734_866,
            },
        );

        // jav
        language_params.insert(
            "jav".to_string(),
            MultilingualParameters {
                char_coef: 0.265_584_2,
                word_coef: 0.212_502_3,
                avg_word_length_coef: 0.666_624_6,
                space_coef: -0.008_391_728,
                intercept: -1.719_723,
            },
        );

        // est
        language_params.insert(
            "est".to_string(),
            MultilingualParameters {
                char_coef: 0.286_972,
                word_coef: 0.217_431_6,
                avg_word_length_coef: 0.307_507_2,
                space_coef: -0.114_979_7,
                intercept: 0.541_236_1,
            },
        );

        // guj
        language_params.insert(
            "guj".to_string(),
            MultilingualParameters {
                char_coef: 0.403_058,
                word_coef: -1.819_486,
                avg_word_length_coef: 0.039_552_81,
                space_coef: 1.641_661,
                intercept: 3.453_395,
            },
        );

        // por
        language_params.insert(
            "por".to_string(),
            MultilingualParameters {
                char_coef: 0.393_995_8,
                word_coef: -0.570_447_9,
                avg_word_length_coef: 0.027_720_06,
                space_coef: -0.202_346_4,
                intercept: 2.029_642,
            },
        );

        // lat
        language_params.insert(
            "lat".to_string(),
            MultilingualParameters {
                char_coef: 0.284_875_5,
                word_coef: -0.059_871_63,
                avg_word_length_coef: 0.643_920_3,
                space_coef: -0.123_747_2,
                intercept: 1.130_221,
            },
        );

        // lit
        language_params.insert(
            "lit".to_string(),
            MultilingualParameters {
                char_coef: 0.290_013_3,
                word_coef: 0.461_793,
                avg_word_length_coef: 0.524_559,
                space_coef: -0.101_460_5,
                intercept: -1.120_401,
            },
        );

        // tha
        language_params.insert(
            "tha".to_string(),
            MultilingualParameters {
                char_coef: 0.401_159_3,
                word_coef: 0.018_947_85,
                avg_word_length_coef: 0.013_625_31,
                space_coef: 0.250_605_8,
                intercept: 1.037_277,
            },
        );

        // vie
        language_params.insert(
            "vie".to_string(),
            MultilingualParameters {
                char_coef: 0.447_339_4,
                word_coef: -0.483_616_2,
                avg_word_length_coef: -0.175_099_8,
                space_coef: -0.153_978_7,
                intercept: 4.175_848,
            },
        );

        // hin
        language_params.insert(
            "hin".to_string(),
            MultilingualParameters {
                char_coef: 0.505_873,
                word_coef: -2.281_346,
                avg_word_length_coef: 0.065_953_92,
                space_coef: 1.314_878,
                intercept: 3.706_578,
            },
        );

        // tam
        language_params.insert(
            "tam".to_string(),
            MultilingualParameters {
                char_coef: 0.292_544_9,
                word_coef: 0.192_996_2,
                avg_word_length_coef: 0.169_438_6,
                space_coef: 0.192_996_2,
                intercept: 1.379_56,
            },
        );

        // slk
        language_params.insert(
            "slk".to_string(),
            MultilingualParameters {
                char_coef: 0.294_144_7,
                word_coef: 0.185_302_6,
                avg_word_length_coef: 0.654_679_2,
                space_coef: 0.109_401,
                intercept: -1.975_32,
            },
        );

        // rus
        language_params.insert(
            "rus".to_string(),
            MultilingualParameters {
                char_coef: 0.209_487_5,
                word_coef: 0.686_485_6,
                avg_word_length_coef: 0.404_388_2,
                space_coef: -0.113_227_4,
                intercept: 2.169_078,
            },
        );

        // mal
        language_params.insert(
            "mal".to_string(),
            MultilingualParameters {
                char_coef: 0.278_958_5,
                word_coef: 0.611_696_6,
                avg_word_length_coef: 0.227_685_9,
                space_coef: 0.037_367_9,
                intercept: 0.275_394_4,
            },
        );

        // khm
        language_params.insert(
            "khm".to_string(),
            MultilingualParameters {
                char_coef: 0.567_05,
                word_coef: 1.827_068,
                avg_word_length_coef: -0.006_425_411,
                space_coef: -2.211_906,
                intercept: -0.305_760_1,
            },
        );

        // mkd
        language_params.insert(
            "mkd".to_string(),
            MultilingualParameters {
                char_coef: 0.266_820_1,
                word_coef: 1.022_644,
                avg_word_length_coef: 0.315_153_4,
                space_coef: -0.595_958_5,
                intercept: -0.084_926_63,
            },
        );

        // jpn
        language_params.insert(
            "jpn".to_string(),
            MultilingualParameters {
                char_coef: 0.731_274_3,
                word_coef: -0.353_474_7,
                avg_word_length_coef: 0.032_461_72,
                space_coef: 0.702_107_6,
                intercept: -0.119_148,
            },
        );

        // dan
        language_params.insert(
            "dan".to_string(),
            MultilingualParameters {
                char_coef: 0.264_389_8,
                word_coef: 0.026_694_61,
                avg_word_length_coef: 0.378_793_5,
                space_coef: 0.029_459,
                intercept: 0.073_488_29,
            },
        );

        // yid
        language_params.insert(
            "yid".to_string(),
            MultilingualParameters {
                char_coef: 0.411_091_2,
                word_coef: -1.475_469,
                avg_word_length_coef: 0.061_718_83,
                space_coef: 1.172_808,
                intercept: 2.833_734,
            },
        );

        // afr
        language_params.insert(
            "afr".to_string(),
            MultilingualParameters {
                char_coef: 0.298_430_7,
                word_coef: 0.346_933_4,
                avg_word_length_coef: 0.463_359_2,
                space_coef: -0.043_252_5,
                intercept: -0.795_372_6,
            },
        );

        // epo
        language_params.insert(
            "epo".to_string(),
            MultilingualParameters {
                char_coef: 0.294_869_3,
                word_coef: 0.109_380_1,
                avg_word_length_coef: 0.657_797_1,
                space_coef: -0.019_217_97,
                intercept: -1.661_745,
            },
        );

        // cat
        language_params.insert(
            "cat".to_string(),
            MultilingualParameters {
                char_coef: 0.231_461_1,
                word_coef: 0.364_809_1,
                avg_word_length_coef: 0.557_608_4,
                space_coef: -0.098_438_06,
                intercept: -0.270_763_4,
            },
        );

        // slv
        language_params.insert(
            "slv".to_string(),
            MultilingualParameters {
                char_coef: 0.259_537_6,
                word_coef: 0.124_060_5,
                avg_word_length_coef: 0.273_143_3,
                space_coef: 0.186_763_8,
                intercept: 0.148_456_8,
            },
        );

        // ron
        language_params.insert(
            "ron".to_string(),
            MultilingualParameters {
                char_coef: 0.185_523_1,
                word_coef: 0.537_310_1,
                avg_word_length_coef: 0.733_326_7,
                space_coef: 0.083_760_6,
                intercept: -1.093_652,
            },
        );

        // spa
        language_params.insert(
            "spa".to_string(),
            MultilingualParameters {
                char_coef: 0.278_155_1,
                word_coef: -0.121_562_9,
                avg_word_length_coef: 0.991_280_6,
                space_coef: -0.003_830_158,
                intercept: -3.071_643,
            },
        );

        // kan
        language_params.insert(
            "kan".to_string(),
            MultilingualParameters {
                char_coef: 0.265_121,
                word_coef: 0.431_834_2,
                avg_word_length_coef: 0.401_911,
                space_coef: 0.431_834_2,
                intercept: -0.550_522_5,
            },
        );

        // bel
        language_params.insert(
            "bel".to_string(),
            MultilingualParameters {
                char_coef: 0.168_973_2,
                word_coef: 1.232_352,
                avg_word_length_coef: 0.611_956_1,
                space_coef: 0.007_597_886,
                intercept: -1.875_359,
            },
        );

        // kat
        language_params.insert(
            "kat".to_string(),
            MultilingualParameters {
                char_coef: 0.219_684_2,
                word_coef: -3.830_661,
                avg_word_length_coef: 0.417_553_1,
                space_coef: 4.654_203,
                intercept: 4.690_637,
            },
        );

        // heb
        language_params.insert(
            "heb".to_string(),
            MultilingualParameters {
                char_coef: 0.448_220_4,
                word_coef: -0.859_212_2,
                avg_word_length_coef: -0.083_739_29,
                space_coef: 0.472_715_1,
                intercept: 2.188_626,
            },
        );

        // hrv
        language_params.insert(
            "hrv".to_string(),
            MultilingualParameters {
                char_coef: 0.226_674_2,
                word_coef: 0.558_613,
                avg_word_length_coef: 0.230_613,
                space_coef: -0.064_203_37,
                intercept: 0.652_11,
            },
        );

        // mya
        language_params.insert(
            "mya".to_string(),
            MultilingualParameters {
                char_coef: 0.550_470_8,
                word_coef: -4.568_54,
                avg_word_length_coef: -0.012_926_1,
                space_coef: 4.074_088,
                intercept: 7.072_039,
            },
        );

        // tur
        language_params.insert(
            "tur".to_string(),
            MultilingualParameters {
                char_coef: 0.131_331_3,
                word_coef: 1.092_517,
                avg_word_length_coef: 1.042_812,
                space_coef: 0.031_549_63,
                intercept: -3.331_118,
            },
        );

        // cmn
        language_params.insert(
            "cmn".to_string(),
            MultilingualParameters {
                char_coef: 0.779_135,
                word_coef: 1.932_796,
                avg_word_length_coef: 0.015_223_59,
                space_coef: -2.791_548,
                intercept: 2.215_65,
            },
        );

        // amh
        language_params.insert(
            "amh".to_string(),
            MultilingualParameters {
                char_coef: 1.658_567,
                word_coef: 0.284_702_8,
                avg_word_length_coef: 0.460_611_8,
                space_coef: 0.284_702_8,
                intercept: -5.909_763,
            },
        );

        // srp
        language_params.insert(
            "srp".to_string(),
            MultilingualParameters {
                char_coef: 0.332_065_4,
                word_coef: -3.506_54,
                avg_word_length_coef: 0.189_467_3,
                space_coef: 3.700_182,
                intercept: 4.844_431,
            },
        );

        // ces
        language_params.insert(
            "ces".to_string(),
            MultilingualParameters {
                char_coef: 0.264_772,
                word_coef: 0.512_141_9,
                avg_word_length_coef: 0.578_131_4,
                space_coef: -0.069_564_6,
                intercept: -1.277_097,
            },
        );

        // nob
        language_params.insert(
            "nob".to_string(),
            MultilingualParameters {
                char_coef: 0.263_612,
                word_coef: 0.027_829_31,
                avg_word_length_coef: 0.385_525_3,
                space_coef: 0.048_874_2,
                intercept: 0.182_105_3,
            },
        );

        // pol
        language_params.insert(
            "pol".to_string(),
            MultilingualParameters {
                char_coef: 0.235_475_1,
                word_coef: 0.552_569_6,
                avg_word_length_coef: 0.743_683_6,
                space_coef: -0.083_838_49,
                intercept: -1.390_128,
            },
        );

        // pan
        language_params.insert(
            "pan".to_string(),
            MultilingualParameters {
                char_coef: 0.720_794,
                word_coef: -0.495_337_2,
                avg_word_length_coef: 0.020_399_82,
                space_coef: -0.495_337_2,
                intercept: 1.183_646,
            },
        );

        // mar
        language_params.insert(
            "mar".to_string(),
            MultilingualParameters {
                char_coef: 0.335_632_3,
                word_coef: -4.067_699,
                avg_word_length_coef: 0.394_379_6,
                space_coef: 4.232_048,
                intercept: 5.148_526,
            },
        );

        // deu
        language_params.insert(
            "deu".to_string(),
            MultilingualParameters {
                char_coef: 0.052_848_1,
                word_coef: 1.029_57,
                avg_word_length_coef: 1.725_175,
                space_coef: 0.132_920_5,
                intercept: -6.397_897,
            },
        );

        // tuk
        language_params.insert(
            "tuk".to_string(),
            MultilingualParameters {
                char_coef: 0.460_565_9,
                word_coef: -0.475_006_3,
                avg_word_length_coef: 0.140_073_7,
                space_coef: -0.283_789_8,
                intercept: 0.815_903_6,
            },
        );

        // pes
        language_params.insert(
            "pes".to_string(),
            MultilingualParameters {
                char_coef: 0.607_078_8,
                word_coef: -0.611_985_9,
                avg_word_length_coef: 0.010_796_54,
                space_coef: -0.611_985_9,
                intercept: 0.325_769_1,
            },
        );

        // tel
        language_params.insert(
            "tel".to_string(),
            MultilingualParameters {
                char_coef: 0.395_741_1,
                word_coef: -1.799_004,
                avg_word_length_coef: 0.111_906_7,
                space_coef: 1.708_118,
                intercept: 3.616_211,
            },
        );

        // uzb
        language_params.insert(
            "uzb".to_string(),
            MultilingualParameters {
                char_coef: 0.286_808_7,
                word_coef: 0.446_137_5,
                avg_word_length_coef: 0.775_441_1,
                space_coef: -0.148_169_8,
                intercept: -3.483_319,
            },
        );

        // zul
        language_params.insert(
            "zul".to_string(),
            MultilingualParameters {
                char_coef: 0.289_738_7,
                word_coef: 0.206_435_3,
                avg_word_length_coef: 0.382_873_4,
                space_coef: -0.026_580_46,
                intercept: -1.111_556,
            },
        );

        // ukr
        language_params.insert(
            "ukr".to_string(),
            MultilingualParameters {
                char_coef: 0.245_843_8,
                word_coef: 0.724_202_7,
                avg_word_length_coef: 0.284_397_7,
                space_coef: -0.101_9,
                intercept: -0.114_443_2,
            },
        );

        // kor
        language_params.insert(
            "kor".to_string(),
            MultilingualParameters {
                char_coef: 0.710_388_9,
                word_coef: -0.263_380_1,
                avg_word_length_coef: 0.245_798,
                space_coef: -0.271_969_5,
                intercept: 1.030_717,
            },
        );

        // bul
        language_params.insert(
            "bul".to_string(),
            MultilingualParameters {
                char_coef: 0.197_806,
                word_coef: 0.913_470_1,
                avg_word_length_coef: 0.575_468,
                space_coef: -0.059_167_12,
                intercept: -1.285_975,
            },
        );

        // aka
        language_params.insert(
            "aka".to_string(),
            MultilingualParameters {
                char_coef: 0.321_864_9,
                word_coef: 0.341_998_8,
                avg_word_length_coef: 0.386_914_3,
                space_coef: -0.164_594_7,
                intercept: -0.932_502,
            },
        );

        // hun
        language_params.insert(
            "hun".to_string(),
            MultilingualParameters {
                char_coef: 0.368_174_4,
                word_coef: -0.048_622_72,
                avg_word_length_coef: 0.175_445_2,
                space_coef: -0.122_741_6,
                intercept: 0.917_479_2,
            },
        );

        // lav
        language_params.insert(
            "lav".to_string(),
            MultilingualParameters {
                char_coef: 0.317_622_9,
                word_coef: -0.436_215_1,
                avg_word_length_coef: 0.307_539,
                space_coef: 0.694_298_6,
                intercept: 0.614_541_2,
            },
        );

        // swe
        language_params.insert(
            "swe".to_string(),
            MultilingualParameters {
                char_coef: 0.288_397_7,
                word_coef: -0.105_335_4,
                avg_word_length_coef: 0.311_394_7,
                space_coef: 0.077_629_13,
                intercept: 0.358_001_9,
            },
        );

        // ori
        language_params.insert(
            "ori".to_string(),
            MultilingualParameters {
                char_coef: 1.043_896,
                word_coef: 20.652_45,
                avg_word_length_coef: -0.347_104_7,
                space_coef: -21.001_68,
                intercept: -19.904_47,
            },
        );

        // urd
        language_params.insert(
            "urd".to_string(),
            MultilingualParameters {
                char_coef: 0.526_020_1,
                word_coef: 0.770_151_9,
                avg_word_length_coef: 0.616_01,
                space_coef: -1.678_662,
                intercept: -2.168_618,
            },
        );

        // ell
        language_params.insert(
            "ell".to_string(),
            MultilingualParameters {
                char_coef: 0.344_482_1,
                word_coef: 0.044_479_94,
                avg_word_length_coef: 0.272_165_5,
                space_coef: 0.044_479_94,
                intercept: 1.355_911,
            },
        );

        // hye
        language_params.insert(
            "hye".to_string(),
            MultilingualParameters {
                char_coef: 0.167_201_5,
                word_coef: 0.515_375_3,
                avg_word_length_coef: 0.742_657,
                space_coef: 0.515_375_3,
                intercept: 0.794_201_7,
            },
        );

        // aze
        language_params.insert(
            "aze".to_string(),
            MultilingualParameters {
                char_coef: 0.150_534_8,
                word_coef: 1.295_725,
                avg_word_length_coef: 0.624_606_3,
                space_coef: -0.047_884_12,
                intercept: -2.082_014,
            },
        );

        Self {
            default_params: MultilingualParameters::default(),
            language_params,
        }
    }
}

pub struct MultilingualMethod {
    parameters: MultilingualMethodParameters,
}

impl MultilingualMethod {
    pub fn new() -> Self {
        Self {
            parameters: MultilingualMethodParameters::default(),
        }
    }
}

impl Default for MultilingualMethod {
    fn default() -> Self {
        Self::new()
    }
}

impl EstimationMethod for MultilingualMethod {
    type Features = MultilingualFeatures;
    type Parameters = MultilingualMethodParameters;

    fn count(&self, text: &str) -> Self::Features {
        // Extract basic features
        let char_count = text.chars().count();
        let space_count = text.chars().filter(|c| c.is_whitespace()).count();
        let words: Vec<&str> = text.split_whitespace().collect();
        let word_count = words.len();

        let avg_word_length = if word_count > 0 {
            let total_word_chars: usize = words.iter().map(|w| w.chars().count()).sum();
            total_word_chars as f32 / word_count as f32
        } else {
            0.0
        };

        // Detect language
        let language = detect(text)
            .map(|info| info.lang().code())
            .unwrap_or("unknown")
            .to_string();

        MultilingualFeatures {
            basic_features: BasicFeatures {
                char_count,
                word_count,
                avg_word_length,
                space_count,
            },
            language,
        }
    }

    fn estimate(&self, text: &str) -> usize {
        let features = self.count(text);

        // Handle empty text
        if features.basic_features.char_count == 0 {
            return 0;
        }

        // Select parameters based on language
        let params = self
            .parameters
            .language_params
            .get(&features.language)
            .unwrap_or(&self.parameters.default_params);

        let bf = &features.basic_features;

        let estimate = params.char_coef * bf.char_count as f32
            + params.word_coef * bf.word_count as f32
            + params.avg_word_length_coef * bf.avg_word_length
            + params.space_coef * bf.space_count as f32
            + params.intercept;

        estimate.round().max(0.0) as usize
    }

    fn parameters(&self) -> Self::Parameters {
        self.parameters.clone()
    }

    fn set_parameters(&mut self, params: Self::Parameters) {
        self.parameters = params;
    }
}
