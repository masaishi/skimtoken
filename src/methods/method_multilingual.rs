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
    pub char_coef: f64,
    pub word_coef: f64,
    pub avg_word_length_coef: f64,
    pub space_coef: f64,
    pub intercept: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultilingualMethodParameters {
    pub default_params: MultilingualParameters,
    pub language_params: HashMap<String, MultilingualParameters>,
}

impl Default for MultilingualParameters {
    fn default() -> Self {
        Self {
            char_coef: 0.3217745347518016,
            word_coef: 0.07022881669049061,
            avg_word_length_coef: 0.5090982427870748,
            space_coef: -0.15831091236345404,
            intercept: 1.591021053665763,
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
                char_coef: 0.44880237128449296,
                word_coef: -18.384453011345162,
                avg_word_length_coef: 1.797303678979317,
                space_coef: 17.888359561907425,
                intercept: 10.365538235307312,
            },
        );

        // ita
        language_params.insert(
            "ita".to_string(),
            MultilingualParameters {
                char_coef: 0.12687422590146652,
                word_coef: 0.7920414147285285,
                avg_word_length_coef: 0.9794865394934512,
                space_coef: 0.1830670312650753,
                intercept: -2.9935769389240576,
            },
        );

        // fin
        language_params.insert(
            "fin".to_string(),
            MultilingualParameters {
                char_coef: 0.2976030333785077,
                word_coef: 0.11046050899116736,
                avg_word_length_coef: 0.1615615124455201,
                space_coef: -0.14307417347906234,
                intercept: 1.4655983525939504,
            },
        );

        // nep
        language_params.insert(
            "nep".to_string(),
            MultilingualParameters {
                char_coef: 0.3420300608969203,
                word_coef: -5.6022718514836605,
                avg_word_length_coef: 0.4029322858107202,
                space_coef: 5.45826703315094,
                intercept: 7.532691006165287,
            },
        );

        // ind
        language_params.insert(
            "ind".to_string(),
            MultilingualParameters {
                char_coef: 0.26697935184155763,
                word_coef: -0.008999044392109063,
                avg_word_length_coef: 0.1554626105076859,
                space_coef: -0.07457480587983964,
                intercept: 2.074445468986177,
            },
        );

        // fra
        language_params.insert(
            "fra".to_string(),
            MultilingualParameters {
                char_coef: 0.1818140837075344,
                word_coef: 0.5483596027473266,
                avg_word_length_coef: 0.6200219566930033,
                space_coef: 0.08400384426485952,
                intercept: -1.556144635894185,
            },
        );

        // nld
        language_params.insert(
            "nld".to_string(),
            MultilingualParameters {
                char_coef: 0.1305321845126117,
                word_coef: 0.39207089340818363,
                avg_word_length_coef: 1.2320205026890658,
                space_coef: 0.4257337692381579,
                intercept: -4.518895273599938,
            },
        );

        // eng
        language_params.insert(
            "eng".to_string(),
            MultilingualParameters {
                char_coef: 0.34793126289383297,
                word_coef: -0.5505503531869712,
                avg_word_length_coef: 0.0792898331331382,
                space_coef: -0.043495655665475384,
                intercept: 2.8539356429595415,
            },
        );

        // tgl
        language_params.insert(
            "tgl".to_string(),
            MultilingualParameters {
                char_coef: 0.36815153196988515,
                word_coef: -0.500993625679746,
                avg_word_length_coef: 0.11735070684592895,
                space_coef: 0.007593871122873626,
                intercept: 1.7558730942283134,
            },
        );

        // ben
        language_params.insert(
            "ben".to_string(),
            MultilingualParameters {
                char_coef: 0.38105771791667237,
                word_coef: 0.015434324737238074,
                avg_word_length_coef: 0.07577055426942005,
                space_coef: 0.0154343247367192,
                intercept: 1.0002898459945868,
            },
        );

        // sin
        language_params.insert(
            "sin".to_string(),
            MultilingualParameters {
                char_coef: 0.521049480687048,
                word_coef: 0.6908051414157408,
                avg_word_length_coef: 0.2480144037015131,
                space_coef: -0.5941421810210131,
                intercept: -1.2452866118864279,
            },
        );

        // sna
        language_params.insert(
            "sna".to_string(),
            MultilingualParameters {
                char_coef: 0.3091881913264699,
                word_coef: -0.03723315055709972,
                avg_word_length_coef: 0.26852864099965157,
                space_coef: -0.13034440813126616,
                intercept: 1.7348661745204517,
            },
        );

        // jav
        language_params.insert(
            "jav".to_string(),
            MultilingualParameters {
                char_coef: 0.26558419532760946,
                word_coef: 0.21250230145944726,
                avg_word_length_coef: 0.6666246189082862,
                space_coef: -0.008391728193954296,
                intercept: -1.7197225022731217,
            },
        );

        // est
        language_params.insert(
            "est".to_string(),
            MultilingualParameters {
                char_coef: 0.2869719682181067,
                word_coef: 0.21743164979545343,
                avg_word_length_coef: 0.30750721110969176,
                space_coef: -0.11497966436688181,
                intercept: 0.5412360708713067,
            },
        );

        // guj
        language_params.insert(
            "guj".to_string(),
            MultilingualParameters {
                char_coef: 0.40305798810057036,
                word_coef: -1.8194864686059908,
                avg_word_length_coef: 0.039552805346301234,
                space_coef: 1.6416614067725386,
                intercept: 3.4533945030777495,
            },
        );

        // por
        language_params.insert(
            "por".to_string(),
            MultilingualParameters {
                char_coef: 0.3939957969751756,
                word_coef: -0.5704479087396972,
                avg_word_length_coef: 0.02772005926440291,
                space_coef: -0.2023463587349651,
                intercept: 2.0296418903088878,
            },
        );

        // lat
        language_params.insert(
            "lat".to_string(),
            MultilingualParameters {
                char_coef: 0.28487551186394927,
                word_coef: -0.059871626232479415,
                avg_word_length_coef: 0.6439203262044544,
                space_coef: -0.12374721098037814,
                intercept: 1.1302209532585152,
            },
        );

        // lit
        language_params.insert(
            "lit".to_string(),
            MultilingualParameters {
                char_coef: 0.2900133213115315,
                word_coef: 0.46179303436512226,
                avg_word_length_coef: 0.5245590300232841,
                space_coef: -0.10146045920399609,
                intercept: -1.1204007147205743,
            },
        );

        // tha
        language_params.insert(
            "tha".to_string(),
            MultilingualParameters {
                char_coef: 0.40115934250902296,
                word_coef: 0.01894784720365421,
                avg_word_length_coef: 0.013625314223821018,
                space_coef: 0.2506058487202946,
                intercept: 1.0372770251835846,
            },
        );

        // vie
        language_params.insert(
            "vie".to_string(),
            MultilingualParameters {
                char_coef: 0.447339411730737,
                word_coef: -0.4836162243503682,
                avg_word_length_coef: -0.17509980690390164,
                space_coef: -0.15397868223774477,
                intercept: 4.175848171991298,
            },
        );

        // hin
        language_params.insert(
            "hin".to_string(),
            MultilingualParameters {
                char_coef: 0.505873006674809,
                word_coef: -2.281346443483259,
                avg_word_length_coef: 0.06595392061707568,
                space_coef: 1.314878214631834,
                intercept: 3.7065777622957654,
            },
        );

        // tam
        language_params.insert(
            "tam".to_string(),
            MultilingualParameters {
                char_coef: 0.2925448667586105,
                word_coef: 0.19299615216604143,
                avg_word_length_coef: 0.1694386249151814,
                space_coef: 0.19299615213383473,
                intercept: 1.3795601376398636,
            },
        );

        // slk
        language_params.insert(
            "slk".to_string(),
            MultilingualParameters {
                char_coef: 0.2941447441098809,
                word_coef: 0.18530255162490036,
                avg_word_length_coef: 0.6546792002540466,
                space_coef: 0.10940097884291243,
                intercept: -1.9753198122471574,
            },
        );

        // rus
        language_params.insert(
            "rus".to_string(),
            MultilingualParameters {
                char_coef: 0.20948745008532185,
                word_coef: 0.6864855781170451,
                avg_word_length_coef: 0.4043882187670413,
                space_coef: -0.11322740917043185,
                intercept: 2.1690779215289098,
            },
        );

        // mal
        language_params.insert(
            "mal".to_string(),
            MultilingualParameters {
                char_coef: 0.27895845310167455,
                word_coef: 0.6116966122570419,
                avg_word_length_coef: 0.22768589015702162,
                space_coef: 0.03736789659060143,
                intercept: 0.27539435822306046,
            },
        );

        // khm
        language_params.insert(
            "khm".to_string(),
            MultilingualParameters {
                char_coef: 0.5670499879446613,
                word_coef: 1.8270677561932331,
                avg_word_length_coef: -0.00642541139331866,
                space_coef: -2.2119058617396052,
                intercept: -0.3057600855326825,
            },
        );

        // mkd
        language_params.insert(
            "mkd".to_string(),
            MultilingualParameters {
                char_coef: 0.26682005050682067,
                word_coef: 1.0226444981752838,
                avg_word_length_coef: 0.3151533873493775,
                space_coef: -0.5959585220198479,
                intercept: -0.08492662593265976,
            },
        );

        // jpn
        language_params.insert(
            "jpn".to_string(),
            MultilingualParameters {
                char_coef: 0.7312743066330536,
                word_coef: -0.3534746595456092,
                avg_word_length_coef: 0.03246171890684391,
                space_coef: 0.7021075662754234,
                intercept: -0.11914804948225566,
            },
        );

        // dan
        language_params.insert(
            "dan".to_string(),
            MultilingualParameters {
                char_coef: 0.26438981337460793,
                word_coef: 0.02669460507817261,
                avg_word_length_coef: 0.3787935341487197,
                space_coef: 0.029458995066212943,
                intercept: 0.07348829239715471,
            },
        );

        // yid
        language_params.insert(
            "yid".to_string(),
            MultilingualParameters {
                char_coef: 0.41109121251495623,
                word_coef: -1.4754686225077989,
                avg_word_length_coef: 0.061718833128964326,
                space_coef: 1.1728084058656487,
                intercept: 2.8337340433802893,
            },
        );

        // afr
        language_params.insert(
            "afr".to_string(),
            MultilingualParameters {
                char_coef: 0.29843067011865443,
                word_coef: 0.34693341723309157,
                avg_word_length_coef: 0.46335921154043186,
                space_coef: -0.043252502229717285,
                intercept: -0.7953726175688445,
            },
        );

        // epo
        language_params.insert(
            "epo".to_string(),
            MultilingualParameters {
                char_coef: 0.2948693287112015,
                word_coef: 0.10938014897914664,
                avg_word_length_coef: 0.6577970638967615,
                space_coef: -0.019217966294145714,
                intercept: -1.661744583613057,
            },
        );

        // cat
        language_params.insert(
            "cat".to_string(),
            MultilingualParameters {
                char_coef: 0.23146113408331725,
                word_coef: 0.364809074790078,
                avg_word_length_coef: 0.5576083944924493,
                space_coef: -0.09843805761578754,
                intercept: -0.2707634413331661,
            },
        );

        // slv
        language_params.insert(
            "slv".to_string(),
            MultilingualParameters {
                char_coef: 0.259537639920391,
                word_coef: 0.1240605166086754,
                avg_word_length_coef: 0.2731433199812429,
                space_coef: 0.18676375105578397,
                intercept: 0.14845683784159291,
            },
        );

        // ron
        language_params.insert(
            "ron".to_string(),
            MultilingualParameters {
                char_coef: 0.18552306511177946,
                word_coef: 0.5373100780374281,
                avg_word_length_coef: 0.7333267314511911,
                space_coef: 0.08376060437747279,
                intercept: -1.093652342393753,
            },
        );

        // spa
        language_params.insert(
            "spa".to_string(),
            MultilingualParameters {
                char_coef: 0.2781551249803241,
                word_coef: -0.1215628532417047,
                avg_word_length_coef: 0.9912805613921394,
                space_coef: -0.003830158436815649,
                intercept: -3.0716427251193323,
            },
        );

        // kan
        language_params.insert(
            "kan".to_string(),
            MultilingualParameters {
                char_coef: 0.26512096863405055,
                word_coef: 0.43183416237836353,
                avg_word_length_coef: 0.40191100549635217,
                space_coef: 0.43183416240596684,
                intercept: -0.550522493216647,
            },
        );

        // bel
        language_params.insert(
            "bel".to_string(),
            MultilingualParameters {
                char_coef: 0.16897318484860901,
                word_coef: 1.232351800470483,
                avg_word_length_coef: 0.6119560807292734,
                space_coef: 0.007597886407324173,
                intercept: -1.8753592228737546,
            },
        );

        // kat
        language_params.insert(
            "kat".to_string(),
            MultilingualParameters {
                char_coef: 0.21968416224127973,
                word_coef: -3.830660989581501,
                avg_word_length_coef: 0.4175531139680374,
                space_coef: 4.654203496988453,
                intercept: 4.690636719048079,
            },
        );

        // heb
        language_params.insert(
            "heb".to_string(),
            MultilingualParameters {
                char_coef: 0.44822042483777935,
                word_coef: -0.8592121723042215,
                avg_word_length_coef: -0.08373929050385838,
                space_coef: 0.47271513192496273,
                intercept: 2.1886263366789933,
            },
        );

        // hrv
        language_params.insert(
            "hrv".to_string(),
            MultilingualParameters {
                char_coef: 0.22667417121583597,
                word_coef: 0.5586130041984836,
                avg_word_length_coef: 0.23061300827316947,
                space_coef: -0.06420336660159277,
                intercept: 0.6521099977338878,
            },
        );

        // mya
        language_params.insert(
            "mya".to_string(),
            MultilingualParameters {
                char_coef: 0.5504708473963755,
                word_coef: -4.568539999054742,
                avg_word_length_coef: -0.012926095483269572,
                space_coef: 4.074088068863251,
                intercept: 7.072039132261878,
            },
        );

        // tur
        language_params.insert(
            "tur".to_string(),
            MultilingualParameters {
                char_coef: 0.13133127958755836,
                word_coef: 1.092516596716582,
                avg_word_length_coef: 1.0428118275412976,
                space_coef: 0.03154963396942866,
                intercept: -3.3311178401423405,
            },
        );

        // cmn
        language_params.insert(
            "cmn".to_string(),
            MultilingualParameters {
                char_coef: 0.779135001698647,
                word_coef: 1.932795715534019,
                avg_word_length_coef: 0.015223590611201137,
                space_coef: -2.791548297576439,
                intercept: 2.2156504738134686,
            },
        );

        // amh
        language_params.insert(
            "amh".to_string(),
            MultilingualParameters {
                char_coef: 1.6585672177924242,
                word_coef: 0.28470277426767054,
                avg_word_length_coef: 0.4606117587101958,
                space_coef: 0.28470277439688263,
                intercept: -5.909762506566722,
            },
        );

        // srp
        language_params.insert(
            "srp".to_string(),
            MultilingualParameters {
                char_coef: 0.33206535331060216,
                word_coef: -3.506539577364872,
                avg_word_length_coef: 0.18946732497918378,
                space_coef: 3.7001815919455714,
                intercept: 4.844430834582951,
            },
        );

        // ces
        language_params.insert(
            "ces".to_string(),
            MultilingualParameters {
                char_coef: 0.2647719635781719,
                word_coef: 0.5121418954770693,
                avg_word_length_coef: 0.5781314454329717,
                space_coef: -0.06956459847424289,
                intercept: -1.277096949134247,
            },
        );

        // nob
        language_params.insert(
            "nob".to_string(),
            MultilingualParameters {
                char_coef: 0.26361204603818883,
                word_coef: 0.027829305702773734,
                avg_word_length_coef: 0.3855253485339995,
                space_coef: 0.048874200778599594,
                intercept: 0.18210533525532213,
            },
        );

        // pol
        language_params.insert(
            "pol".to_string(),
            MultilingualParameters {
                char_coef: 0.23547508240710424,
                word_coef: 0.552569596879804,
                avg_word_length_coef: 0.7436836480046146,
                space_coef: -0.08383848984047836,
                intercept: -1.390127615026465,
            },
        );

        // pan
        language_params.insert(
            "pan".to_string(),
            MultilingualParameters {
                char_coef: 0.7207940197614274,
                word_coef: -0.4953372252610426,
                avg_word_length_coef: 0.020399824305131504,
                space_coef: -0.4953372252976133,
                intercept: 1.18364572663274,
            },
        );

        // mar
        language_params.insert(
            "mar".to_string(),
            MultilingualParameters {
                char_coef: 0.3356322556815121,
                word_coef: -4.067699296123827,
                avg_word_length_coef: 0.394379569694404,
                space_coef: 4.232047770284952,
                intercept: 5.148526184518431,
            },
        );

        // deu
        language_params.insert(
            "deu".to_string(),
            MultilingualParameters {
                char_coef: 0.05284810432351846,
                word_coef: 1.0295701332097542,
                avg_word_length_coef: 1.7251745782227788,
                space_coef: 0.1329204901978866,
                intercept: -6.397896513911476,
            },
        );

        // tuk
        language_params.insert(
            "tuk".to_string(),
            MultilingualParameters {
                char_coef: 0.4605658953745427,
                word_coef: -0.4750062562283548,
                avg_word_length_coef: 0.1400736882727119,
                space_coef: -0.28378978190390125,
                intercept: 0.8159035787470046,
            },
        );

        // pes
        language_params.insert(
            "pes".to_string(),
            MultilingualParameters {
                char_coef: 0.6070787706869065,
                word_coef: -0.6119858863321161,
                avg_word_length_coef: 0.010796540462907489,
                space_coef: -0.6119858868808072,
                intercept: 0.32576914380694433,
            },
        );

        // tel
        language_params.insert(
            "tel".to_string(),
            MultilingualParameters {
                char_coef: 0.39574110882952507,
                word_coef: -1.7990041443407216,
                avg_word_length_coef: 0.11190671366518141,
                space_coef: 1.7081182724544473,
                intercept: 3.6162105455993867,
            },
        );

        // uzb
        language_params.insert(
            "uzb".to_string(),
            MultilingualParameters {
                char_coef: 0.2868086900215845,
                word_coef: 0.4461375130186268,
                avg_word_length_coef: 0.775441126607956,
                space_coef: -0.14816977926204605,
                intercept: -3.4833190686188615,
            },
        );

        // zul
        language_params.insert(
            "zul".to_string(),
            MultilingualParameters {
                char_coef: 0.28973871900540854,
                word_coef: 0.20643529746018757,
                avg_word_length_coef: 0.38287335109774945,
                space_coef: -0.026580462937346314,
                intercept: -1.1115560814115142,
            },
        );

        // ukr
        language_params.insert(
            "ukr".to_string(),
            MultilingualParameters {
                char_coef: 0.24584382517512468,
                word_coef: 0.7242026917438468,
                avg_word_length_coef: 0.28439773465355883,
                space_coef: -0.10189998135109683,
                intercept: -0.11444320642682015,
            },
        );

        // kor
        language_params.insert(
            "kor".to_string(),
            MultilingualParameters {
                char_coef: 0.7103889270864719,
                word_coef: -0.2633801082236818,
                avg_word_length_coef: 0.2457979940622727,
                space_coef: -0.27196949734726356,
                intercept: 1.0307168628604373,
            },
        );

        // bul
        language_params.insert(
            "bul".to_string(),
            MultilingualParameters {
                char_coef: 0.19780598879510816,
                word_coef: 0.9134701308361612,
                avg_word_length_coef: 0.5754679703149507,
                space_coef: -0.059167115284642954,
                intercept: -1.2859754825008025,
            },
        );

        // aka
        language_params.insert(
            "aka".to_string(),
            MultilingualParameters {
                char_coef: 0.3218648650819916,
                word_coef: 0.34199876587973327,
                avg_word_length_coef: 0.3869142530775766,
                space_coef: -0.16459474235465021,
                intercept: -0.9325020266183657,
            },
        );

        // hun
        language_params.insert(
            "hun".to_string(),
            MultilingualParameters {
                char_coef: 0.36817443587826887,
                word_coef: -0.048622719888274324,
                avg_word_length_coef: 0.17544524998894495,
                space_coef: -0.12274164543498649,
                intercept: 0.9174791651131713,
            },
        );

        // lav
        language_params.insert(
            "lav".to_string(),
            MultilingualParameters {
                char_coef: 0.3176229303661853,
                word_coef: -0.4362151085246064,
                avg_word_length_coef: 0.3075390457271601,
                space_coef: 0.6942985689623983,
                intercept: 0.6145412064183944,
            },
        );

        // swe
        language_params.insert(
            "swe".to_string(),
            MultilingualParameters {
                char_coef: 0.2883977188397274,
                word_coef: -0.10533538963823742,
                avg_word_length_coef: 0.3113946738361182,
                space_coef: 0.0776291340603509,
                intercept: 0.3580018702339487,
            },
        );

        // ori
        language_params.insert(
            "ori".to_string(),
            MultilingualParameters {
                char_coef: 1.0438964753605617,
                word_coef: 20.652451733769315,
                avg_word_length_coef: -0.3471047122979283,
                space_coef: -21.001684886151132,
                intercept: -19.904470694839674,
            },
        );

        // urd
        language_params.insert(
            "urd".to_string(),
            MultilingualParameters {
                char_coef: 0.5260200912923618,
                word_coef: 0.7701518847126705,
                avg_word_length_coef: 0.6160099739394632,
                space_coef: -1.678662232679409,
                intercept: -2.1686180269571693,
            },
        );

        // ell
        language_params.insert(
            "ell".to_string(),
            MultilingualParameters {
                char_coef: 0.344482080176014,
                word_coef: 0.0444799412340854,
                avg_word_length_coef: 0.27216550228791514,
                space_coef: 0.04447994122047855,
                intercept: 1.355911194086218,
            },
        );

        // hye
        language_params.insert(
            "hye".to_string(),
            MultilingualParameters {
                char_coef: 0.16720147167253888,
                word_coef: 0.5153752531236651,
                avg_word_length_coef: 0.742656992721823,
                space_coef: 0.5153752531060737,
                intercept: 0.7942016664529916,
            },
        );

        // aze
        language_params.insert(
            "aze".to_string(),
            MultilingualParameters {
                char_coef: 0.1505348428193723,
                word_coef: 1.2957249331467364,
                avg_word_length_coef: 0.6246063179631899,
                space_coef: -0.04788411831550755,
                intercept: -2.082014490647552,
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
            total_word_chars as f64 / word_count as f64
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

        let estimate = params.char_coef * bf.char_count as f64
            + params.word_coef * bf.word_count as f64
            + params.avg_word_length_coef * bf.avg_word_length
            + params.space_coef * bf.space_count as f64
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
