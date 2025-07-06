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
            char_coef: 0.32180722194872413,
            word_coef: 0.14000829322945874,
            avg_word_length_coef: 0.5089246172388747,
            space_coef: -0.22833568099621965,
            intercept: 1.523645960367098,
        }
    }
}

impl Default for MultilingualMethodParameters {
    fn default() -> Self {
        let mut language_params = HashMap::new();

        // Ara
        language_params.insert(
            "Ara".to_string(),
            MultilingualParameters {
                char_coef: 0.4489517902904245,
                word_coef: -0.5348165257580506,
                avg_word_length_coef: 1.7963490564468942,
                space_coef: 0.03796899891041773,
                intercept: -7.481054644431325,
            },
        );

        // Ita
        language_params.insert(
            "Ita".to_string(),
            MultilingualParameters {
                char_coef: 0.12915668052218623,
                word_coef: 1.0863455753534303,
                avg_word_length_coef: 0.9287645274129221,
                space_coef: -0.1259048925945162,
                intercept: -3.0044184405485908,
            },
        );

        // Fin
        language_params.insert(
            "Fin".to_string(),
            MultilingualParameters {
                char_coef: 0.2976193485393542,
                word_coef: 0.11825654353480729,
                avg_word_length_coef: 0.16149040743426094,
                space_coef: -0.15100832557663063,
                intercept: 1.4584106047996457,
            },
        );

        // Nep
        language_params.insert(
            "Nep".to_string(),
            MultilingualParameters {
                char_coef: 0.34238997940017374,
                word_coef: 0.6301674543044935,
                avg_word_length_coef: 0.40272882433354734,
                space_coef: -0.7774614070861645,
                intercept: 1.3284146635226932,
            },
        );

        // Ind
        language_params.insert(
            "Ind".to_string(),
            MultilingualParameters {
                char_coef: 0.26699771462127964,
                word_coef: 0.00872492998198966,
                avg_word_length_coef: 0.15539103582948802,
                space_coef: -0.09243440359654641,
                intercept: 2.05728312826448,
            },
        );

        // Fra
        language_params.insert(
            "Fra".to_string(),
            MultilingualParameters {
                char_coef: 0.18184136983909963,
                word_coef: 0.5844140684643012,
                avg_word_length_coef: 0.6199515252044037,
                space_coef: 0.04778065587017218,
                intercept: -1.591360576323055,
            },
        );

        // Nld
        language_params.insert(
            "Nld".to_string(),
            MultilingualParameters {
                char_coef: 0.1305686027494848,
                word_coef: 0.42006733639670985,
                avg_word_length_coef: 1.2314695291171138,
                space_coef: 0.3974934464308953,
                intercept: -4.542935241775886,
            },
        );

        // Eng
        language_params.insert(
            "Eng".to_string(),
            MultilingualParameters {
                char_coef: 0.34794096093051735,
                word_coef: -0.5427642998380915,
                avg_word_length_coef: 0.07924010836821016,
                space_coef: -0.05134346816243043,
                intercept: 2.846585958883164,
            },
        );

        // Tgl
        language_params.insert(
            "Tgl".to_string(),
            MultilingualParameters {
                char_coef: 0.3682661500096258,
                word_coef: -0.4113369910327406,
                avg_word_length_coef: 0.11659546549993971,
                space_coef: -0.08285826260928329,
                intercept: 1.6723334721949996,
            },
        );

        // Ben
        language_params.insert(
            "Ben".to_string(),
            MultilingualParameters {
                char_coef: 0.38109691637881826,
                word_coef: 2.4979104248910295,
                avg_word_length_coef: 0.07566671739200109,
                space_coef: -2.467325159953596,
                intercept: -1.4818706057080782,
            },
        );

        // Sin
        language_params.insert(
            "Sin".to_string(),
            MultilingualParameters {
                char_coef: 0.5210550306852654,
                word_coef: 0.048316936845774726,
                avg_word_length_coef: 0.24782097162748584,
                space_coef: 0.04831693684251858,
                intercept: -0.6020805383800649,
            },
        );

        // Sna
        language_params.insert(
            "Sna".to_string(),
            MultilingualParameters {
                char_coef: 0.30920302503240105,
                word_coef: -0.033129426490317,
                avg_word_length_coef: 0.26848297347511013,
                space_coef: -0.13454559749762535,
                intercept: 1.7308658603455065,
            },
        );

        // Jav
        language_params.insert(
            "Jav".to_string(),
            MultilingualParameters {
                char_coef: 0.265739992368337,
                word_coef: 0.3842240768493444,
                avg_word_length_coef: 0.6646478357009543,
                space_coef: -0.18125960800933189,
                intercept: -1.8753583538376972,
            },
        );

        // Est
        language_params.insert(
            "Est".to_string(),
            MultilingualParameters {
                char_coef: 0.2867849353837958,
                word_coef: 0.17818601093961448,
                avg_word_length_coef: 0.30847339314681527,
                space_coef: -0.07414330061944695,
                intercept: 0.5681131326821145,
            },
        );

        // Guj
        language_params.insert(
            "Guj".to_string(),
            MultilingualParameters {
                char_coef: 0.4030879402121555,
                word_coef: -0.027772186092496844,
                avg_word_length_coef: 0.039507893020558085,
                space_coef: -0.15039709202001944,
                intercept: 1.668836837440253,
            },
        );

        // Por
        language_params.insert(
            "Por".to_string(),
            MultilingualParameters {
                char_coef: 0.39394217973877277,
                word_coef: -0.5591723804295806,
                avg_word_length_coef: 0.027602540255544115,
                space_coef: -0.21329795095033288,
                intercept: 2.01997355781279,
            },
        );

        // Lat
        language_params.insert(
            "Lat".to_string(),
            MultilingualParameters {
                char_coef: 0.28491307199131405,
                word_coef: -0.017981090754277777,
                avg_word_length_coef: 0.6435726095800065,
                space_coef: -0.1659264299946576,
                intercept: 1.0913711096732683,
            },
        );

        // Lit
        language_params.insert(
            "Lit".to_string(),
            MultilingualParameters {
                char_coef: 0.290015929360577,
                word_coef: 0.46179084022048333,
                avg_word_length_coef: 0.5245380940226348,
                space_coef: -0.10147774867868146,
                intercept: -1.1202853051721817,
            },
        );

        // Tha
        language_params.insert(
            "Tha".to_string(),
            MultilingualParameters {
                char_coef: 0.40103041289312835,
                word_coef: 1.870284436832706,
                avg_word_length_coef: 0.013664356961895661,
                space_coef: -1.599099814001211,
                intercept: -0.8146666569739693,
            },
        );

        // Vie
        language_params.insert(
            "Vie".to_string(),
            MultilingualParameters {
                char_coef: 0.44736187930002136,
                word_coef: -0.23442394376782635,
                avg_word_length_coef: -0.1753780325446561,
                space_coef: -0.4032859743336601,
                intercept: 3.928208894288055,
            },
        );

        // Hin
        language_params.insert(
            "Hin".to_string(),
            MultilingualParameters {
                char_coef: 0.5057492935452412,
                word_coef: -4.371087647636833,
                avg_word_length_coef: 0.06517253619745014,
                space_coef: 3.4052749375480658,
                intercept: 5.798711157449148,
            },
        );

        // Tam
        language_params.insert(
            "Tam".to_string(),
            MultilingualParameters {
                char_coef: 0.29254786483909,
                word_coef: 0.19298284845071445,
                avg_word_length_coef: 0.16942112741820717,
                space_coef: 0.19298284843277144,
                intercept: 1.3796926185568985,
            },
        );

        // Slk
        language_params.insert(
            "Slk".to_string(),
            MultilingualParameters {
                char_coef: 0.29414846761857655,
                word_coef: 0.18848777172243622,
                avg_word_length_coef: 0.6545934183764769,
                space_coef: 0.10618949511360945,
                intercept: -1.977904796186337,
            },
        );

        // Rus
        language_params.insert(
            "Rus".to_string(),
            MultilingualParameters {
                char_coef: 0.20951034514560174,
                word_coef: 0.681566127420288,
                avg_word_length_coef: 0.4044341512544039,
                space_coef: -0.1084751243972746,
                intercept: 2.173346161737989,
            },
        );

        // Mal
        language_params.insert(
            "Mal".to_string(),
            MultilingualParameters {
                char_coef: 0.2789605167974779,
                word_coef: 0.5159309741225317,
                avg_word_length_coef: 0.22768676564613982,
                space_coef: 0.13311359768676181,
                intercept: 0.3711041895854237,
            },
        );

        // Khm
        language_params.insert(
            "Khm".to_string(),
            MultilingualParameters {
                char_coef: 0.5671108414344348,
                word_coef: 0.09167119942605235,
                avg_word_length_coef: -0.006501965852451086,
                space_coef: -0.47785357796043304,
                intercept: 1.4291427306743643,
            },
        );

        // Mkd
        language_params.insert(
            "Mkd".to_string(),
            MultilingualParameters {
                char_coef: 0.26682431944743695,
                word_coef: 0.8876732906024092,
                avg_word_length_coef: 0.3151174178315572,
                space_coef: -0.461011859193001,
                intercept: 0.05005352515232886,
            },
        );

        // Jpn
        language_params.insert(
            "Jpn".to_string(),
            MultilingualParameters {
                char_coef: 0.7262105470760017,
                word_coef: 1.6219284677655668,
                avg_word_length_coef: 0.037694008323224816,
                space_coef: -1.3968058491956772,
                intercept: -2.105511887291513,
            },
        );

        // Dan
        language_params.insert(
            "Dan".to_string(),
            MultilingualParameters {
                char_coef: 0.2646024835227419,
                word_coef: 0.12090349476472527,
                avg_word_length_coef: 0.3779372122048349,
                space_coef: -0.06610741859279971,
                intercept: -0.013374035450741673,
            },
        );

        // Yid
        language_params.insert(
            "Yid".to_string(),
            MultilingualParameters {
                char_coef: 0.4110855962747689,
                word_coef: -1.1811235779407048,
                avg_word_length_coef: 0.061714329093150944,
                space_coef: 0.8784942210567511,
                intercept: 2.5396025725862756,
            },
        );

        // Afr
        language_params.insert(
            "Afr".to_string(),
            MultilingualParameters {
                char_coef: 0.2984717903564894,
                word_coef: 0.39043881025638105,
                avg_word_length_coef: 0.4630853859090543,
                space_coef: -0.0870184513794742,
                intercept: -0.8366675517089703,
            },
        );

        // Epo
        language_params.insert(
            "Epo".to_string(),
            MultilingualParameters {
                char_coef: 0.2949706195633288,
                word_coef: 0.15069698505674184,
                avg_word_length_coef: 0.6567045180981311,
                space_coef: -0.061197843794602906,
                intercept: -1.69630826827445,
            },
        );

        // Cat
        language_params.insert(
            "Cat".to_string(),
            MultilingualParameters {
                char_coef: 0.23148614967246667,
                word_coef: 0.3760225923701342,
                avg_word_length_coef: 0.5573614563849949,
                space_coef: -0.10980945440153264,
                intercept: -0.2804791150708823,
            },
        );

        // Slv
        language_params.insert(
            "Slv".to_string(),
            MultilingualParameters {
                char_coef: 0.2595371755924374,
                word_coef: 0.2119468378982711,
                avg_word_length_coef: 0.27299624777451137,
                space_coef: 0.0988742784994839,
                intercept: 0.061793034569710414,
            },
        );

        // Ron
        language_params.insert(
            "Ron".to_string(),
            MultilingualParameters {
                char_coef: 0.18552795033444947,
                word_coef: 0.5381740301613079,
                avg_word_length_coef: 0.7332506813043093,
                space_coef: 0.08286273674386783,
                intercept: -1.09395742429394,
            },
        );

        // Spa
        language_params.insert(
            "Spa".to_string(),
            MultilingualParameters {
                char_coef: 0.2782168228332222,
                word_coef: -0.11631376798675855,
                avg_word_length_coef: 0.9898457737647731,
                space_coef: -0.009481546696791089,
                intercept: -3.0685681407127063,
            },
        );

        // Kan
        language_params.insert(
            "Kan".to_string(),
            MultilingualParameters {
                char_coef: 0.26521405886715027,
                word_coef: 2.2907288564005728,
                avg_word_length_coef: 0.4014119662886503,
                space_coef: -1.4279574522261076,
                intercept: -2.4062480350569615,
            },
        );

        // Bel
        language_params.insert(
            "Bel".to_string(),
            MultilingualParameters {
                char_coef: 0.16897899381218381,
                word_coef: 1.2385614048974807,
                avg_word_length_coef: 0.6118707755443012,
                space_coef: 0.001344537306719022,
                intercept: -1.8810225246761902,
            },
        );

        // Kat
        language_params.insert(
            "Kat".to_string(),
            MultilingualParameters {
                char_coef: 0.21973757052176138,
                word_coef: -3.1621336915309173,
                avg_word_length_coef: 0.4174105533708153,
                space_coef: 3.985248693915196,
                intercept: 4.024715778006147,
            },
        );

        // Heb
        language_params.insert(
            "Heb".to_string(),
            MultilingualParameters {
                char_coef: 0.4481860140099636,
                word_coef: -0.7248619988417391,
                avg_word_length_coef: -0.0835295746967152,
                space_coef: 0.33857763847199795,
                intercept: 2.0532791093536105,
            },
        );

        // Hrv
        language_params.insert(
            "Hrv".to_string(),
            MultilingualParameters {
                char_coef: 0.2266826439156682,
                word_coef: 0.5609797671434241,
                avg_word_length_coef: 0.2305417442744601,
                space_coef: -0.06662503469548993,
                intercept: 0.6501027669968025,
            },
        );

        // Mya
        language_params.insert(
            "Mya".to_string(),
            MultilingualParameters {
                char_coef: 0.5504528579411492,
                word_coef: -4.362024125497104,
                avg_word_length_coef: -0.012908283431363018,
                space_coef: 3.867799397392941,
                intercept: 6.865765198898913,
            },
        );

        // Tur
        language_params.insert(
            "Tur".to_string(),
            MultilingualParameters {
                char_coef: 0.13141484139581816,
                word_coef: 1.1154956052649132,
                avg_word_length_coef: 1.0419610417713725,
                space_coef: 0.00791540843910715,
                intercept: -3.3476513173756217,
            },
        );

        // Cmn
        language_params.insert(
            "Cmn".to_string(),
            MultilingualParameters {
                char_coef: 0.7789858977206273,
                word_coef: -0.7223268306000218,
                avg_word_length_coef: 0.015416762157169321,
                space_coef: -0.13472471423148935,
                intercept: 4.86372547066334,
            },
        );

        // Amh
        language_params.insert(
            "Amh".to_string(),
            MultilingualParameters {
                char_coef: 1.6585699891453178,
                word_coef: 0.28469555138578945,
                avg_word_length_coef: 0.46053526966382397,
                space_coef: 0.28469555142391084,
                intercept: -5.909449211416444,
            },
        );

        // Srp
        language_params.insert(
            "Srp".to_string(),
            MultilingualParameters {
                char_coef: 0.3320580995603634,
                word_coef: -3.4891819747891546,
                avg_word_length_coef: 0.18962659412316554,
                space_coef: 3.6828604701186176,
                intercept: 4.827053580757244,
            },
        );

        // Ces
        language_params.insert(
            "Ces".to_string(),
            MultilingualParameters {
                char_coef: 0.26564736829446023,
                word_coef: 0.5769147105576441,
                avg_word_length_coef: 0.5537596867956732,
                space_coef: -0.1399440202045154,
                intercept: -1.2117707808221923,
            },
        );

        // Nob
        language_params.insert(
            "Nob".to_string(),
            MultilingualParameters {
                char_coef: 0.2636354892212806,
                word_coef: 0.08490124873765849,
                avg_word_length_coef: 0.38520782381791446,
                space_coef: -0.008363599597556755,
                intercept: 0.1278796273630718,
            },
        );

        // Pol
        language_params.insert(
            "Pol".to_string(),
            MultilingualParameters {
                char_coef: 0.23533665666219675,
                word_coef: 0.5627655291172652,
                avg_word_length_coef: 0.7433486374076463,
                space_coef: -0.09309825979216961,
                intercept: -1.397406125997641,
            },
        );

        // Pan
        language_params.insert(
            "Pan".to_string(),
            MultilingualParameters {
                char_coef: 0.720789380582767,
                word_coef: -0.4953254914887084,
                avg_word_length_coef: 0.020408280723599403,
                space_coef: -0.49532549147411525,
                intercept: 1.1836230712234794,
            },
        );

        // Mar
        language_params.insert(
            "Mar".to_string(),
            MultilingualParameters {
                char_coef: 0.33569533307671573,
                word_coef: -3.356625868205123,
                avg_word_length_coef: 0.39470542680458987,
                space_coef: 3.5204536641049016,
                intercept: 4.439645487554493,
            },
        );

        // Deu
        language_params.insert(
            "Deu".to_string(),
            MultilingualParameters {
                char_coef: 0.052462519372940615,
                word_coef: 1.049888224787395,
                avg_word_length_coef: 1.736765497549089,
                space_coef: 0.11515089672218608,
                intercept: -6.480230484835545,
            },
        );

        // Tuk
        language_params.insert(
            "Tuk".to_string(),
            MultilingualParameters {
                char_coef: 0.46064274278795436,
                word_coef: -0.4741729743920428,
                avg_word_length_coef: 0.1400116808340529,
                space_coef: -0.2850756894150074,
                intercept: 0.8146447693519114,
            },
        );

        // Pes
        language_params.insert(
            "Pes".to_string(),
            MultilingualParameters {
                char_coef: 0.6070711053761995,
                word_coef: 2.4339476094299277,
                avg_word_length_coef: 0.010493363598046607,
                space_coef: -3.6578731249431162,
                intercept: -2.7198752750327415,
            },
        );

        // Tel
        language_params.insert(
            "Tel".to_string(),
            MultilingualParameters {
                char_coef: 0.3957425419250069,
                word_coef: -1.483090028742508,
                avg_word_length_coef: 0.11190074912729651,
                space_coef: 1.3922065072729741,
                intercept: 3.3004046282261044,
            },
        );

        // Uzb
        language_params.insert(
            "Uzb".to_string(),
            MultilingualParameters {
                char_coef: 0.2869198490831655,
                word_coef: 0.47593046196977545,
                avg_word_length_coef: 0.7749681354127103,
                space_coef: -0.1788978319183927,
                intercept: -3.509703934908572,
            },
        );

        // Zul
        language_params.insert(
            "Zul".to_string(),
            MultilingualParameters {
                char_coef: 0.2897482282610496,
                word_coef: 0.23785369050902963,
                avg_word_length_coef: 0.38259613632516054,
                space_coef: -0.05808106064535399,
                intercept: -1.1406204673102351,
            },
        );

        // Ukr
        language_params.insert(
            "Ukr".to_string(),
            MultilingualParameters {
                char_coef: 0.24593753698020412,
                word_coef: 0.7819415366892607,
                avg_word_length_coef: 0.2803201713568763,
                space_coef: -0.16025347362500747,
                intercept: -0.15116381857311012,
            },
        );

        // Kor
        language_params.insert(
            "Kor".to_string(),
            MultilingualParameters {
                char_coef: 0.7105375227576862,
                word_coef: -0.12655494069566567,
                avg_word_length_coef: 0.24556543712620946,
                space_coef: -0.40944024497266657,
                intercept: 0.8939994850945467,
            },
        );

        // Bul
        language_params.insert(
            "Bul".to_string(),
            MultilingualParameters {
                char_coef: 0.1978106077159651,
                word_coef: 0.9133446083282273,
                avg_word_length_coef: 0.57536886890713,
                space_coef: -0.05907041773828037,
                intercept: -1.2853426576559883,
            },
        );

        // Aka
        language_params.insert(
            "Aka".to_string(),
            MultilingualParameters {
                char_coef: 0.3218727043632269,
                word_coef: 0.3430592757275351,
                avg_word_length_coef: 0.38651056519023713,
                space_coef: -0.16567535756936555,
                intercept: -0.9318833181650028,
            },
        );

        // Hun
        language_params.insert(
            "Hun".to_string(),
            MultilingualParameters {
                char_coef: 0.3694500820833435,
                word_coef: 0.012182804312283484,
                avg_word_length_coef: 0.13500732558733686,
                space_coef: -0.19268347055407936,
                intercept: 1.1064567269166403,
            },
        );

        // Lav
        language_params.insert(
            "Lav".to_string(),
            MultilingualParameters {
                char_coef: 0.3176361510912947,
                word_coef: -0.4156651847767781,
                avg_word_length_coef: 0.30710569211010674,
                space_coef: 0.6736352281181303,
                intercept: 0.5975359650387944,
            },
        );

        // Swe
        language_params.insert(
            "Swe".to_string(),
            MultilingualParameters {
                char_coef: 0.2883965674464913,
                word_coef: -0.10549257546209627,
                avg_word_length_coef: 0.3115528323842523,
                space_coef: 0.07779297587035676,
                intercept: 0.35738006649609844,
            },
        );

        // Ori
        language_params.insert(
            "Ori".to_string(),
            MultilingualParameters {
                char_coef: 1.0439492382508784,
                word_coef: 18.56543096584324,
                avg_word_length_coef: -0.34672898744006336,
                space_coef: -18.914978230429824,
                intercept: -17.82513364910325,
            },
        );

        // Urd
        language_params.insert(
            "Urd".to_string(),
            MultilingualParameters {
                char_coef: 0.5260217869866554,
                word_coef: 0.28037642045986144,
                avg_word_length_coef: 0.6159336248208801,
                space_coef: -1.1888948241529904,
                intercept: -1.6785931400218388,
            },
        );

        // Ell
        language_params.insert(
            "Ell".to_string(),
            MultilingualParameters {
                char_coef: 0.3444837752964474,
                word_coef: 0.04447436638881558,
                avg_word_length_coef: 0.27211696954218,
                space_coef: 0.04447436639105847,
                intercept: 1.356178126197804,
            },
        );

        // Hye
        language_params.insert(
            "Hye".to_string(),
            MultilingualParameters {
                char_coef: 0.1672046742260253,
                word_coef: 0.515362953483478,
                avg_word_length_coef: 0.7425981159253168,
                space_coef: 0.5153629534599142,
                intercept: 0.7945928422724222,
            },
        );

        // Aze
        language_params.insert(
            "Aze".to_string(),
            MultilingualParameters {
                char_coef: 0.15054996518067584,
                word_coef: 1.2966992775535442,
                avg_word_length_coef: 0.6243652121283592,
                space_coef: -0.048975428203563626,
                intercept: -2.0814220114345474,
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
