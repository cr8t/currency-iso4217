//! ISO 4217 <https://en.wikipedia.org/wiki/ISO_4217> currency codes
//!
//! A `no_std` implementation of ISO 4217 currency codes.
//!
//! Aims to be as minimalistic as possible, while still providing a full set of features for
//! handling ISO 4217 currency codes.
//!
//! Alternatives:
//!
//! - [`rust-iso4217`](https://github.com/rust-iso/rust_iso4217)
//! - [`codes-iso-4217`](https://github.com/johnstonskj/rust-codes)
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[cfg(not(feature = "std"))]
use core as std;
#[cfg(feature = "std")]
use std;

use std::fmt;

/// ISO 4217 currency codes <https://en.wikipedia.org/wiki/ISO_4217>
#[repr(u32)]
#[rustfmt::skip]
#[allow(clippy::zero_prefixed_literal)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Currency {
    /// United Arab Emirates dirham United Arab Emirates
    AED = 784,
    /// Afghan afghani Afghanistan
    AFN = 971,
    /// Albanian lek Albania
    ALL = 008,
    /// Armenian dram Armenia
    AMD = 051,
    /// Netherlands Antillean guilder Cura
    ANG = 532,
    /// Angolan kwanza Angola
    AOA = 973,
    /// Argentine peso Argentina
    ARS = 032,
    /// Australian dollar Australia
    AUD = 036,
    /// Aruban florin Aruba
    AWG = 533,
    /// Azerbaijani manat Azerbaijan
    AZN = 944,
    /// Bosnia and Herzegovina convertible mark Bosnia and Herzegovina
    BAM = 977,
    /// Barbados dollar Barbados
    BBD = 052,
    /// Bangladeshi taka Bangladesh
    BDT = 050,
    /// Bulgarian lev Bulgaria
    BGN = 975,
    /// Bahraini dinar Bahrain
    BHD = 048,
    /// Burundian franc Burundi
    BIF = 108,
    /// Bermudian dollar Bermuda
    BMD = 060,
    /// Brunei dollar Brunei
    BND = 096,
    /// Boliviano Bolivia
    BOB = 068,
    /// Bolivian Mvdol (funds code)
    BOV = 984,
    /// Brazilian real Brazil
    BRL = 986,
    /// Bahamian dollar Bahamas
    BSD = 044,
    /// Bhutanese ngultrum Bhutan
    BTN = 064,
    /// Botswana pula Botswana
    BWP = 072,
    /// Belarusian ruble Belarus
    BYN = 933,
    /// Belize dollar Belize
    BZD = 084,
    /// Canadian dollar Canada
    CAD = 124,
    /// Congolese franc Democratic Republic of the Congo
    CDF = 976,
    /// WIR euro (complementary currency)
    CHE = 947,
    /// Swiss franc Switzerland
    CHF = 756,
    /// WIR franc (complementary currency)
    CHW = 948,
    /// Unidad de Fomento (funds code)
    CLF = 990,
    /// Chilean peso Chile
    CLP = 152,
    /// Colombian peso Colombia
    COP = 170,
    ///  Unidad de Valor Real (UVR) (funds code)
    COU = 970,
    /// Costa Rican colon Costa Rica
    CRC = 188,
    /// Cuban convertible peso Cuba
    CUC = 931,
    /// Cuban peso Cuba
    CUP = 192,
    /// Cape Verdean escudo Cabo Verde
    CVE = 132,
    /// Czech koruna Czechia
    CZK = 203,
    /// Djiboutian franc Djibouti
    DJF = 262,
    /// Danish krone Denmark
    DKK = 208,
    /// Dominican peso Dominican Republic
    DOP = 214,
    /// Algerian dinar Algeria
    DZD = 012,
    /// Egyptian pound Egypt
    EGP = 818,
    /// Eritrean nakfa Eritrea
    ERN = 232,
    /// Ethiopian birr Ethiopia
    ETB = 230,
    /// Euro
    EUR = 978,
    /// Fiji dollar Fiji
    FJD = 242,
    /// Falkland Islands pound Falkland Islands (pegged to GBP)
    FKP = 238,
    /// Pound sterling United Kingdom
    GBP = 826,
    /// Georgian lari Georgia
    GEL = 981,
    /// Ghanaian cedi Ghana
    GHS = 936,
    /// Gibraltar pound Gibraltar (pegged to GBP)
    GIP = 292,
    /// Gambian dalasi Gambia
    GMD = 270,
    /// Guinean franc Guinea
    GNF = 324,
    /// Guatemalan quetzal Guatemala
    GTQ = 320,
    /// Guyanese dollar Guyana
    GYD = 328,
    /// Hong Kong dollar Hong Kong
    HKD = 344,
    /// Honduran lempira Honduras
    HNL = 340,
    /// Haitian gourde Haiti
    HTG = 332,
    /// Hungarian forint Hungary
    HUF = 348,
    /// Indonesian rupiah Indonesia
    IDR = 360,
    /// Israeli new shekel Israel
    ILS = 376,
    /// Indian rupee India
    INR = 356,
    /// Iraqi dinar Iraq
    IQD = 368,
    /// Iranian rial Iran
    IRR = 364,
    /// Icelandic kr
    ISK = 352,
    /// Jamaican dollar Jamaica
    JMD = 388,
    /// Jordanian dinar Jordan
    JOD = 400,
    /// Japanese yen Japan
    JPY = 392,
    /// Kenyan shilling Kenya
    KES = 404,
    /// Kyrgyzstani som Kyrgyzstan
    KGS = 417,
    /// Cambodian riel Cambodia
    KHR = 116,
    /// Comoro franc Comoros
    KMF = 174,
    /// North Korean won North Korea
    KPW = 408,
    /// South Korean won
    KRW = 410,
    /// Kuwaiti dinar Kuwait
    KWD = 414,
    /// Cayman Islands dollar Cayman Islands
    KYD = 136,
    /// Kazakhstani tenge Kazakhstan
    KZT = 398,
    /// Lao kip Laos
    LAK = 418,
    /// Lebanese pound Lebanon
    LBP = 422,
    /// Sri Lankan rupee Sri Lanka
    LKR = 144,
    /// Liberian dollar Liberia
    LRD = 430,
    /// Lesotho loti Lesotho
    LSL = 426,
    /// Libyan dinar Libya
    LYD = 434,
    /// Moroccan dirham Morocco
    MAD = 504,
    /// Moldovan leu Moldova
    MDL = 498,
    /// Malagasy ariary
    MGA = 969,
    /// Macedonian denar North Macedonia
    MKD = 807,
    /// Myanmar kyat Myanmar
    MMK = 104,
    /// Mongolian tögrög
    MNT = 496,
    /// Macanese pataca Macau
    MOP = 446,
    /// Mauritanian ouguiya
    MRU = 929,
    /// Mauritian rupee Mauritius
    MUR = 480,
    /// Maldivian rufiyaa Maldives
    MVR = 462,
    /// Malawian kwacha Malawi
    MWK = 454,
    /// Mexican peso Mexico
    MXN = 484,
    /// Mexican Unidad de Inversion (UDI) (funds code)
    MXV = 979,
    /// Malaysian ringgit Malaysia
    MYR = 458,
    /// Mozambican metical Mozambique
    MZN = 943,
    /// Namibian dollar Namibia (pegged to ZAR)
    NAD = 516,
    /// Nigerian naira Nigeria
    NGN = 566,
    /// Nicaraguan c
    NIO = 558,
    /// Norwegian krone Norway
    NOK = 578,
    /// Nepalese rupee Nepal
    NPR = 524,
    /// New Zealand dollar New Zealand
    NZD = 554,
    /// Omani rial Oman
    OMR = 512,
    /// Panamanian balboa Panama
    PAB = 590,
    /// Peruvian sol Peru
    PEN = 604,
    /// Papua New Guinean kina Papua New Guinea
    PGK = 598,
    /// Philippine peso
    PHP = 608,
    /// Pakistani rupee Pakistan
    PKR = 586,
    /// Polish z
    PLN = 985,
    /// Paraguayan guaran
    PYG = 600,
    /// Qatari riyal Qatar
    QAR = 634,
    /// Romanian leu Romania
    RON = 946,
    /// Serbian dinar Serbia
    RSD = 941,
    /// Renminbi
    CNY = 156,
    /// Russian ruble Russia
    RUB = 643,
    /// Rwandan franc Rwanda
    RWF = 646,
    /// Saudi riyal Saudi Arabia
    SAR = 682,
    /// Solomon Islands dollar Solomon Islands
    SBD = 090,
    /// Seychelles rupee Seychelles
    SCR = 690,
    /// Sudanese pound Sudan
    SDG = 938,
    /// Swedish krona (plural
    SEK = 752,
    /// Singapore dollar Singapore
    SGD = 702,
    /// Saint Helena pound Saint Helena (SH
    SHP = 654,
    /// Sierra Leonean leone (new leone)
    SLE = 925,
    /// Sierra Leonean leone (old leone)
    SLL = 694,
    /// Somali shilling Somalia
    SOS = 706,
    /// Surinamese dollar Suriname
    SRD = 968,
    /// South Sudanese pound South Sudan
    SSP = 728,
    /// São Tomé and Príncipe dobra
    STN = 930,
    /// Salvadoran col
    SVC = 222,
    /// Syrian pound Syria
    SYP = 760,
    /// Swazi lilangeni Eswatini
    SZL = 748,
    /// Thai baht Thailand
    THB = 764,
    /// Tajikistani somoni Tajikistan
    TJS = 972,
    /// Turkmenistan manat Turkmenistan
    TMT = 934,
    /// Tunisian dinar Tunisia
    TND = 788,
    /// Tongan pa
    TOP = 776,
    /// Turkish lira Turkey
    TRY = 949,
    /// Trinidad and Tobago dollar Trinidad and Tobago
    TTD = 780,
    /// New Taiwan dollar Taiwan
    TWD = 901,
    /// Tanzanian shilling Tanzania
    TZS = 834,
    /// Ukrainian hryvnia Ukraine
    UAH = 980,
    /// Ugandan shilling Uganda
    UGX = 800,
    /// United States dollar United States
    USD = 840,
    /// United States dollar (next day) (funds code)
    USN = 997,
    /// Uruguay Peso en Unidades Indexadas (URUIURUI) (funds code)
    UYI = 940,
    /// Uruguayan peso Uruguay
    UYU = 858,
    /// Unidad previsional
    UYW = 927,
    /// Uzbekistan sum Uzbekistan
    UZS = 860,
    /// Venezuelan digital bol
    VED = 926,
    /// Venezuelan sovereign bol
    VES = 928,
    /// Vietnamese
    VND = 704,
    /// Vanuatu vatu Vanuatu
    VUV = 548,
    /// Samoan tala Samoa
    WST = 882,
    /// CFA franc BEAC Cameroon (CM)
    XAF = 950,
    /// Silver (one troy ounce)
    XAG = 961,
    /// Gold (one troy ounce)
    XAU = 959,
    /// European Composite Unit (bond market unit)
    XBA = 955,
    /// European Monetary Unit (bond market unit)
    XBB = 956,
    /// European Unit of Account 9 (bond market unit)
    XBC = 957,
    /// European unit of account 17 (bond market unit)
    XBD = 958,
    /// East Caribbean dollar Anguilla (AI)
    XCD = 951,
    /// Special drawing rights
    XDR = 960,
    /// CFA franc BCEAO Benin (BJ)
    XOF = 952,
    /// Palladium (one troy ounce)
    XPD = 964,
    /// CFP franc (franc Pacifique)
    XPF = 953,
    /// Platinum (one troy ounce)
    XPT = 962,
    /// SUCRE
    XSU = 994,
    /// Code reserved for testing
    XTS = 963,
    /// ADB Unit of Account
    XUA = 965,
    /// No country code
    #[default]
    XXX = 999,
    /// Yemeni rial Yemen
    YER = 886,
    /// South African rand Eswatini
    ZAR = 710,
    /// Zambian kwacha Zambia
    ZMW = 967,
    /// Zimbabwean dollar (fifth)
    ZWL = 932,
}

impl Currency {
    /// The length of the ASCII string, not the internal representation.
    pub const LEN: usize = 3;

    /// Creates a new [Currency].
    pub const fn new() -> Self {
        Self::XXX
    }

    /// Gets the name of the currency (in English).
    pub const fn name(&self) -> &'static str {
        match self {
            Self::AED => "United Arab Emirates dirham United Arab Emirates",
            Self::AFN => "Afghan afghani Afghanistan",
            Self::ALL => "Albanian lek Albania",
            Self::AMD => "Armenian dram Armenia",
            Self::ANG => "Netherlands Antillean guilder Cura",
            Self::AOA => "Angolan kwanza Angola",
            Self::ARS => "Argentine peso Argentina",
            Self::AUD => "Australian dollar Australia",
            Self::AWG => "Aruban florin Aruba",
            Self::AZN => "Azerbaijani manat Azerbaijan",
            Self::BAM => "Bosnia and Herzegovina convertible mark Bosnia and Herzegovina",
            Self::BBD => "Barbados dollar Barbados",
            Self::BDT => "Bangladeshi taka Bangladesh",
            Self::BGN => "Bulgarian lev Bulgaria",
            Self::BHD => "Bahraini dinar Bahrain",
            Self::BIF => "Burundian franc Burundi",
            Self::BMD => "Bermudian dollar Bermuda",
            Self::BND => "Brunei dollar Brunei",
            Self::BOB => "Boliviano Bolivia",
            Self::BOV => "Bolivian Mvdol (funds code)",
            Self::BRL => "Brazilian real Brazil",
            Self::BSD => "Bahamian dollar Bahamas",
            Self::BTN => "Bhutanese ngultrum Bhutan",
            Self::BWP => "Botswana pula Botswana",
            Self::BYN => "Belarusian ruble Belarus",
            Self::BZD => "Belize dollar Belize",
            Self::CAD => "Canadian dollar Canada",
            Self::CDF => "Congolese franc Democratic Republic of the Congo",
            Self::CHE => "WIR euro (complementary currency)",
            Self::CHF => "Swiss franc Switzerland",
            Self::CHW => "WIR franc (complementary currency)",
            Self::CLF => "Unidad de Fomento (funds code)",
            Self::CLP => "Chilean peso Chile",
            Self::COP => "Colombian peso Colombia",
            Self::COU => " Unidad de Valor Real (UVR) (funds code)",
            Self::CRC => "Costa Rican colon Costa Rica",
            Self::CUC => "Cuban convertible peso Cuba",
            Self::CUP => "Cuban peso Cuba",
            Self::CVE => "Cape Verdean escudo Cabo Verde",
            Self::CZK => "Czech koruna Czechia",
            Self::DJF => "Djiboutian franc Djibouti",
            Self::DKK => "Danish krone Denmark",
            Self::DOP => "Dominican peso Dominican Republic",
            Self::DZD => "Algerian dinar Algeria",
            Self::EGP => "Egyptian pound Egypt",
            Self::ERN => "Eritrean nakfa Eritrea",
            Self::ETB => "Ethiopian birr Ethiopia",
            Self::EUR => "Euro",
            Self::FJD => "Fiji dollar Fiji",
            Self::FKP => "Falkland Islands pound Falkland Islands (pegged to GBP)",
            Self::GBP => "Pound sterling United Kingdom",
            Self::GEL => "Georgian lari Georgia",
            Self::GHS => "Ghanaian cedi Ghana",
            Self::GIP => "Gibraltar pound Gibraltar (pegged to GBP)",
            Self::GMD => "Gambian dalasi Gambia",
            Self::GNF => "Guinean franc Guinea",
            Self::GTQ => "Guatemalan quetzal Guatemala",
            Self::GYD => "Guyanese dollar Guyana",
            Self::HKD => "Hong Kong dollar Hong Kong",
            Self::HNL => "Honduran lempira Honduras",
            Self::HTG => "Haitian gourde Haiti",
            Self::HUF => "Hungarian forint Hungary",
            Self::IDR => "Indonesian rupiah Indonesia",
            Self::ILS => "Israeli new shekel Israel",
            Self::INR => "Indian rupee India",
            Self::IQD => "Iraqi dinar Iraq",
            Self::IRR => "Iranian rial Iran",
            Self::ISK => "Icelandic kr",
            Self::JMD => "Jamaican dollar Jamaica",
            Self::JOD => "Jordanian dinar Jordan",
            Self::JPY => "Japanese yen Japan",
            Self::KES => "Kenyan shilling Kenya",
            Self::KGS => "Kyrgyzstani som Kyrgyzstan",
            Self::KHR => "Cambodian riel Cambodia",
            Self::KMF => "Comoro franc Comoros",
            Self::KPW => "North Korean won North Korea",
            Self::KRW => "South Korean won",
            Self::KWD => "Kuwaiti dinar Kuwait",
            Self::KYD => "Cayman Islands dollar Cayman Islands",
            Self::KZT => "Kazakhstani tenge Kazakhstan",
            Self::LAK => "Lao kip Laos",
            Self::LBP => "Lebanese pound Lebanon",
            Self::LKR => "Sri Lankan rupee Sri Lanka",
            Self::LRD => "Liberian dollar Liberia",
            Self::LSL => "Lesotho loti Lesotho",
            Self::LYD => "Libyan dinar Libya",
            Self::MAD => "Moroccan dirham Morocco",
            Self::MDL => "Moldovan leu Moldova",
            Self::MGA => "Malagasy ariary",
            Self::MKD => "Macedonian denar North Macedonia",
            Self::MMK => "Myanmar kyat Myanmar",
            Self::MNT => "Mongolian tögrög",
            Self::MOP => "Macanese pataca Macau",
            Self::MRU => "Mauritanian ouguiya",
            Self::MUR => "Mauritian rupee Mauritius",
            Self::MVR => "Maldivian rufiyaa Maldives",
            Self::MWK => "Malawian kwacha Malawi",
            Self::MXN => "Mexican peso Mexico",
            Self::MXV => "Mexican Unidad de Inversion (UDI) (funds code)",
            Self::MYR => "Malaysian ringgit Malaysia",
            Self::MZN => "Mozambican metical Mozambique",
            Self::NAD => "Namibian dollar Namibia (pegged to ZAR)",
            Self::NGN => "Nigerian naira Nigeria",
            Self::NIO => "Nicaraguan c",
            Self::NOK => "Norwegian krone Norway",
            Self::NPR => "Nepalese rupee Nepal",
            Self::NZD => "New Zealand dollar New Zealand",
            Self::OMR => "Omani rial Oman",
            Self::PAB => "Panamanian balboa Panama",
            Self::PEN => "Peruvian sol Peru",
            Self::PGK => "Papua New Guinean kina Papua New Guinea",
            Self::PHP => "Philippine peso",
            Self::PKR => "Pakistani rupee Pakistan",
            Self::PLN => "Polish z",
            Self::PYG => "Paraguayan guaran",
            Self::QAR => "Qatari riyal Qatar",
            Self::RON => "Romanian leu Romania",
            Self::RSD => "Serbian dinar Serbia",
            Self::CNY => "Renminbi",
            Self::RUB => "Russian ruble Russia",
            Self::RWF => "Rwandan franc Rwanda",
            Self::SAR => "Saudi riyal Saudi Arabia",
            Self::SBD => "Solomon Islands dollar Solomon Islands",
            Self::SCR => "Seychelles rupee Seychelles",
            Self::SDG => "Sudanese pound Sudan",
            Self::SEK => "Swedish krona (plural",
            Self::SGD => "Singapore dollar Singapore",
            Self::SHP => "Saint Helena pound Saint Helena (SH",
            Self::SLE => "Sierra Leonean leone (new leone)",
            Self::SLL => "Sierra Leonean leone (old leone)",
            Self::SOS => "Somali shilling Somalia",
            Self::SRD => "Surinamese dollar Suriname",
            Self::SSP => "South Sudanese pound South Sudan",
            Self::STN => "São Tomé and Príncipe dobra",
            Self::SVC => "Salvadoran col",
            Self::SYP => "Syrian pound Syria",
            Self::SZL => "Swazi lilangeni Eswatini",
            Self::THB => "Thai baht Thailand",
            Self::TJS => "Tajikistani somoni Tajikistan",
            Self::TMT => "Turkmenistan manat Turkmenistan",
            Self::TND => "Tunisian dinar Tunisia",
            Self::TOP => "Tongan pa",
            Self::TRY => "Turkish lira Turkey",
            Self::TTD => "Trinidad and Tobago dollar Trinidad and Tobago",
            Self::TWD => "New Taiwan dollar Taiwan",
            Self::TZS => "Tanzanian shilling Tanzania",
            Self::UAH => "Ukrainian hryvnia Ukraine",
            Self::UGX => "Ugandan shilling Uganda",
            Self::USD => "United States dollar United States",
            Self::USN => "United States dollar (next day) (funds code)",
            Self::UYI => "Uruguay Peso en Unidades Indexadas (URUIURUI) (funds code)",
            Self::UYU => "Uruguayan peso Uruguay",
            Self::UYW => "Unidad previsional",
            Self::UZS => "Uzbekistan sum Uzbekistan",
            Self::VED => "Venezuelan digital bol",
            Self::VES => "Venezuelan sovereign bol",
            Self::VND => "Vietnamese",
            Self::VUV => "Vanuatu vatu Vanuatu",
            Self::WST => "Samoan tala Samoa",
            Self::XAF => "CFA franc BEAC Cameroon (CM)",
            Self::XAG => "Silver (one troy ounce)",
            Self::XAU => "Gold (one troy ounce)",
            Self::XBA => "European Composite Unit (bond market unit)",
            Self::XBB => "European Monetary Unit (bond market unit)",
            Self::XBC => "European Unit of Account 9 (bond market unit)",
            Self::XBD => "European unit of account 17 (bond market unit)",
            Self::XCD => "East Caribbean dollar Anguilla (AI)",
            Self::XDR => "Special drawing rights",
            Self::XOF => "CFA franc BCEAO Benin (BJ)",
            Self::XPD => "Palladium (one troy ounce)",
            Self::XPF => "CFP franc (franc Pacifique)",
            Self::XPT => "Platinum (one troy ounce)",
            Self::XSU => "SUCRE",
            Self::XTS => "Code reserved for testing",
            Self::XUA => "ADB Unit of Account",
            Self::XXX => "No country code",
            Self::YER => "Yemeni rial Yemen",
            Self::ZAR => "South African rand Eswatini",
            Self::ZMW => "Zambian kwacha Zambia",
            Self::ZWL => "Zimbabwean dollar (fifth)",
        }
    }
}

impl From<Currency> for u32 {
    fn from(val: Currency) -> Self {
        val as u32
    }
}

impl From<&Currency> for u32 {
    fn from(val: &Currency) -> Self {
        (*val).into()
    }
}

impl From<Currency> for [i8; 4] {
    fn from(val: Currency) -> Self {
        let cur_str = <&str>::from(val);
        let cur_bytes = cur_str.as_bytes();

        [
            cur_bytes[0] as i8,
            cur_bytes[1] as i8,
            cur_bytes[2] as i8,
            b'\0' as i8,
        ]
    }
}

impl From<&Currency> for [i8; 4] {
    fn from(val: &Currency) -> Self {
        (*val).into()
    }
}

impl From<Currency> for &'static str {
    fn from(val: Currency) -> Self {
        match val {
            Currency::AED => "AED",
            Currency::AFN => "AFN",
            Currency::ALL => "ALL",
            Currency::AMD => "AMD",
            Currency::ANG => "ANG",
            Currency::AOA => "AOA",
            Currency::ARS => "ARS",
            Currency::AUD => "AUD",
            Currency::AWG => "AWG",
            Currency::AZN => "AZN",
            Currency::BAM => "BAM",
            Currency::BBD => "BBD",
            Currency::BDT => "BDT",
            Currency::BGN => "BGN",
            Currency::BHD => "BHD",
            Currency::BIF => "BIF",
            Currency::BMD => "BMD",
            Currency::BND => "BND",
            Currency::BOB => "BOB",
            Currency::BOV => "BOV",
            Currency::BRL => "BRL",
            Currency::BSD => "BSD",
            Currency::BTN => "BTN",
            Currency::BWP => "BWP",
            Currency::BYN => "BYN",
            Currency::BZD => "BZD",
            Currency::CAD => "CAD",
            Currency::CDF => "CDF",
            Currency::CHE => "CHE",
            Currency::CHF => "CHF",
            Currency::CHW => "CHW",
            Currency::CLF => "CLF",
            Currency::CLP => "CLP",
            Currency::COP => "COP",
            Currency::COU => "COU",
            Currency::CRC => "CRC",
            Currency::CUC => "CUC",
            Currency::CUP => "CUP",
            Currency::CVE => "CVE",
            Currency::CZK => "CZK",
            Currency::DJF => "DJF",
            Currency::DKK => "DKK",
            Currency::DOP => "DOP",
            Currency::DZD => "DZD",
            Currency::EGP => "EGP",
            Currency::ERN => "ERN",
            Currency::ETB => "ETB",
            Currency::EUR => "EUR",
            Currency::FJD => "FJD",
            Currency::FKP => "FKP",
            Currency::GBP => "GBP",
            Currency::GEL => "GEL",
            Currency::GHS => "GHS",
            Currency::GIP => "GIP",
            Currency::GMD => "GMD",
            Currency::GNF => "GNF",
            Currency::GTQ => "GTQ",
            Currency::GYD => "GYD",
            Currency::HKD => "HKD",
            Currency::HNL => "HNL",
            Currency::HTG => "HTG",
            Currency::HUF => "HUF",
            Currency::IDR => "IDR",
            Currency::ILS => "ILS",
            Currency::INR => "INR",
            Currency::IQD => "IQD",
            Currency::IRR => "IRR",
            Currency::ISK => "ISK",
            Currency::JMD => "JMD",
            Currency::JOD => "JOD",
            Currency::JPY => "JPY",
            Currency::KES => "KES",
            Currency::KGS => "KGS",
            Currency::KHR => "KHR",
            Currency::KMF => "KMF",
            Currency::KPW => "KPW",
            Currency::KRW => "KRW",
            Currency::KWD => "KWD",
            Currency::KYD => "KYD",
            Currency::KZT => "KZT",
            Currency::LAK => "LAK",
            Currency::LBP => "LBP",
            Currency::LKR => "LKR",
            Currency::LRD => "LRD",
            Currency::LSL => "LSL",
            Currency::LYD => "LYD",
            Currency::MAD => "MAD",
            Currency::MDL => "MDL",
            Currency::MGA => "MGA",
            Currency::MKD => "MKD",
            Currency::MMK => "MMK",
            Currency::MNT => "MNT",
            Currency::MOP => "MOP",
            Currency::MRU => "MRU",
            Currency::MUR => "MUR",
            Currency::MVR => "MVR",
            Currency::MWK => "MWK",
            Currency::MXN => "MXN",
            Currency::MXV => "MXV",
            Currency::MYR => "MYR",
            Currency::MZN => "MZN",
            Currency::NAD => "NAD",
            Currency::NGN => "NGN",
            Currency::NIO => "NIO",
            Currency::NOK => "NOK",
            Currency::NPR => "NPR",
            Currency::NZD => "NZD",
            Currency::OMR => "OMR",
            Currency::PAB => "PAB",
            Currency::PEN => "PEN",
            Currency::PGK => "PGK",
            Currency::PHP => "PHP",
            Currency::PKR => "PKR",
            Currency::PLN => "PLN",
            Currency::PYG => "PYG",
            Currency::QAR => "QAR",
            Currency::RON => "RON",
            Currency::RSD => "RSD",
            Currency::CNY => "CNY",
            Currency::RUB => "RUB",
            Currency::RWF => "RWF",
            Currency::SAR => "SAR",
            Currency::SBD => "SBD",
            Currency::SCR => "SCR",
            Currency::SDG => "SDG",
            Currency::SEK => "SEK",
            Currency::SGD => "SGD",
            Currency::SHP => "SHP",
            Currency::SLE => "SLE",
            Currency::SLL => "SLL",
            Currency::SOS => "SOS",
            Currency::SRD => "SRD",
            Currency::SSP => "SSP",
            Currency::STN => "STN",
            Currency::SVC => "SVC",
            Currency::SYP => "SYP",
            Currency::SZL => "SZL",
            Currency::THB => "THB",
            Currency::TJS => "TJS",
            Currency::TMT => "TMT",
            Currency::TND => "TND",
            Currency::TOP => "TOP",
            Currency::TRY => "TRY",
            Currency::TTD => "TTD",
            Currency::TWD => "TWD",
            Currency::TZS => "TZS",
            Currency::UAH => "UAH",
            Currency::UGX => "UGX",
            Currency::USD => "USD",
            Currency::USN => "USN",
            Currency::UYI => "UYI",
            Currency::UYU => "UYU",
            Currency::UYW => "UYW",
            Currency::UZS => "UZS",
            Currency::VED => "VED",
            Currency::VES => "VES",
            Currency::VND => "VND",
            Currency::VUV => "VUV",
            Currency::WST => "WST",
            Currency::XAF => "XAF",
            Currency::XAG => "XAG",
            Currency::XAU => "XAU",
            Currency::XBA => "XBA",
            Currency::XBB => "XBB",
            Currency::XBC => "XBC",
            Currency::XBD => "XBD",
            Currency::XCD => "XCD",
            Currency::XDR => "XDR",
            Currency::XOF => "XOF",
            Currency::XPD => "XPD",
            Currency::XPF => "XPF",
            Currency::XPT => "XPT",
            Currency::XSU => "XSU",
            Currency::XTS => "XTS",
            Currency::XUA => "XUA",
            Currency::XXX => "XXX",
            Currency::YER => "YER",
            Currency::ZAR => "ZAR",
            Currency::ZMW => "ZMW",
            Currency::ZWL => "ZWL",
        }
    }
}

impl From<&Currency> for &'static str {
    fn from(val: &Currency) -> Self {
        (*val).into()
    }
}

impl From<&str> for Currency {
    fn from(val: &str) -> Self {
        if val.len() < 3 {
            Self::XXX
        } else {
            let cc = val.to_uppercase();
            match &cc[..3] {
                "AED" => Self::AED,
                "AFN" => Self::AFN,
                "ALL" => Self::ALL,
                "AMD" => Self::AMD,
                "ANG" => Self::ANG,
                "AOA" => Self::AOA,
                "ARS" => Self::ARS,
                "AUD" => Self::AUD,
                "AWG" => Self::AWG,
                "AZN" => Self::AZN,
                "BAM" => Self::BAM,
                "BBD" => Self::BBD,
                "BDT" => Self::BDT,
                "BGN" => Self::BGN,
                "BHD" => Self::BHD,
                "BIF" => Self::BIF,
                "BMD" => Self::BMD,
                "BND" => Self::BND,
                "BOB" => Self::BOB,
                "BOV" => Self::BOV,
                "BRL" => Self::BRL,
                "BSD" => Self::BSD,
                "BTN" => Self::BTN,
                "BWP" => Self::BWP,
                "BYN" => Self::BYN,
                "BZD" => Self::BZD,
                "CAD" => Self::CAD,
                "CDF" => Self::CDF,
                "CHE" => Self::CHE,
                "CHF" => Self::CHF,
                "CHW" => Self::CHW,
                "CLF" => Self::CLF,
                "CLP" => Self::CLP,
                "COP" => Self::COP,
                "COU" => Self::COU,
                "CRC" => Self::CRC,
                "CUC" => Self::CUC,
                "CUP" => Self::CUP,
                "CVE" => Self::CVE,
                "CZK" => Self::CZK,
                "DJF" => Self::DJF,
                "DKK" => Self::DKK,
                "DOP" => Self::DOP,
                "DZD" => Self::DZD,
                "EGP" => Self::EGP,
                "ERN" => Self::ERN,
                "ETB" => Self::ETB,
                "EUR" => Self::EUR,
                "FJD" => Self::FJD,
                "FKP" => Self::FKP,
                "GBP" => Self::GBP,
                "GEL" => Self::GEL,
                "GHS" => Self::GHS,
                "GIP" => Self::GIP,
                "GMD" => Self::GMD,
                "GNF" => Self::GNF,
                "GTQ" => Self::GTQ,
                "GYD" => Self::GYD,
                "HKD" => Self::HKD,
                "HNL" => Self::HNL,
                "HTG" => Self::HTG,
                "HUF" => Self::HUF,
                "IDR" => Self::IDR,
                "ILS" => Self::ILS,
                "INR" => Self::INR,
                "IQD" => Self::IQD,
                "IRR" => Self::IRR,
                "ISK" => Self::ISK,
                "JMD" => Self::JMD,
                "JOD" => Self::JOD,
                "JPY" => Self::JPY,
                "KES" => Self::KES,
                "KGS" => Self::KGS,
                "KHR" => Self::KHR,
                "KMF" => Self::KMF,
                "KPW" => Self::KPW,
                "KRW" => Self::KRW,
                "KWD" => Self::KWD,
                "KYD" => Self::KYD,
                "KZT" => Self::KZT,
                "LAK" => Self::LAK,
                "LBP" => Self::LBP,
                "LKR" => Self::LKR,
                "LRD" => Self::LRD,
                "LSL" => Self::LSL,
                "LYD" => Self::LYD,
                "MAD" => Self::MAD,
                "MDL" => Self::MDL,
                "MGA" => Self::MGA,
                "MKD" => Self::MKD,
                "MMK" => Self::MMK,
                "MNT" => Self::MNT,
                "MOP" => Self::MOP,
                "MRU" => Self::MRU,
                "MUR" => Self::MUR,
                "MVR" => Self::MVR,
                "MWK" => Self::MWK,
                "MXN" => Self::MXN,
                "MXV" => Self::MXV,
                "MYR" => Self::MYR,
                "MZN" => Self::MZN,
                "NAD" => Self::NAD,
                "NGN" => Self::NGN,
                "NIO" => Self::NIO,
                "NOK" => Self::NOK,
                "NPR" => Self::NPR,
                "NZD" => Self::NZD,
                "OMR" => Self::OMR,
                "PAB" => Self::PAB,
                "PEN" => Self::PEN,
                "PGK" => Self::PGK,
                "PHP" => Self::PHP,
                "PKR" => Self::PKR,
                "PLN" => Self::PLN,
                "PYG" => Self::PYG,
                "QAR" => Self::QAR,
                "RON" => Self::RON,
                "RSD" => Self::RSD,
                "CNY" => Self::CNY,
                "RUB" => Self::RUB,
                "RWF" => Self::RWF,
                "SAR" => Self::SAR,
                "SBD" => Self::SBD,
                "SCR" => Self::SCR,
                "SDG" => Self::SDG,
                "SEK" => Self::SEK,
                "SGD" => Self::SGD,
                "SHP" => Self::SHP,
                "SLE" => Self::SLE,
                "SLL" => Self::SLL,
                "SOS" => Self::SOS,
                "SRD" => Self::SRD,
                "SSP" => Self::SSP,
                "STN" => Self::STN,
                "SVC" => Self::SVC,
                "SYP" => Self::SYP,
                "SZL" => Self::SZL,
                "THB" => Self::THB,
                "TJS" => Self::TJS,
                "TMT" => Self::TMT,
                "TND" => Self::TND,
                "TOP" => Self::TOP,
                "TRY" => Self::TRY,
                "TTD" => Self::TTD,
                "TWD" => Self::TWD,
                "TZS" => Self::TZS,
                "UAH" => Self::UAH,
                "UGX" => Self::UGX,
                "USD" => Self::USD,
                "USN" => Self::USN,
                "UYI" => Self::UYI,
                "UYU" => Self::UYU,
                "UYW" => Self::UYW,
                "UZS" => Self::UZS,
                "VED" => Self::VED,
                "VES" => Self::VES,
                "VND" => Self::VND,
                "VUV" => Self::VUV,
                "WST" => Self::WST,
                "XAF" => Self::XAF,
                "XAG" => Self::XAG,
                "XAU" => Self::XAU,
                "XBA" => Self::XBA,
                "XBB" => Self::XBB,
                "XBC" => Self::XBC,
                "XBD" => Self::XBD,
                "XCD" => Self::XCD,
                "XDR" => Self::XDR,
                "XOF" => Self::XOF,
                "XPD" => Self::XPD,
                "XPF" => Self::XPF,
                "XPT" => Self::XPT,
                "XSU" => Self::XSU,
                "XTS" => Self::XTS,
                "XUA" => Self::XUA,
                "XXX" => Self::XXX,
                "YER" => Self::YER,
                "ZAR" => Self::ZAR,
                "ZMW" => Self::ZMW,
                "ZWL" => Self::ZWL,
                _ => Self::XXX,
            }
        }
    }
}

impl From<&[i8]> for Currency {
    fn from(val: &[i8]) -> Self {
        if val.len() < 3 {
            Self::XXX
        } else {
            std::str::from_utf8(&[val[0] as u8, val[1] as u8, val[2] as u8])
                .unwrap_or("XXX")
                .into()
        }
    }
}

impl<const N: usize> From<&[i8; N]> for Currency {
    fn from(val: &[i8; N]) -> Self {
        val.as_ref().into()
    }
}

impl<const N: usize> From<[i8; N]> for Currency {
    fn from(val: [i8; N]) -> Self {
        val.as_ref().into()
    }
}

impl From<&[u8]> for Currency {
    fn from(val: &[u8]) -> Self {
        if val.len() < 3 {
            Self::XXX
        } else {
            std::str::from_utf8(&[val[0], val[1], val[2]])
                .unwrap_or("XXX")
                .into()
        }
    }
}

impl<const N: usize> From<&[u8; N]> for Currency {
    fn from(val: &[u8; N]) -> Self {
        val.as_ref().into()
    }
}

impl<const N: usize> From<[u8; N]> for Currency {
    fn from(val: [u8; N]) -> Self {
        val.as_ref().into()
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}
