use std::{collections::HashMap, convert::TryFrom};

use crate::core::{
    BLPAPI_DEFAULT_ALL, BLPAPI_DEFAULT_REALTIME, BLPAPI_DEFAULT_STATIC,
    BLPAPI_LNG_OVERRIDE_CHINESE_SIMP, BLPAPI_LNG_OVERRIDE_CHINESE_TRAD,
    BLPAPI_LNG_OVERRIDE_ENGLISH, BLPAPI_LNG_OVERRIDE_FRENCH, BLPAPI_LNG_OVERRIDE_GERMAN,
    BLPAPI_LNG_OVERRIDE_ITALIAN, BLPAPI_LNG_OVERRIDE_KANJI, BLPAPI_LNG_OVERRIDE_KOREAN,
    BLPAPI_LNG_OVERRIDE_NONE, BLPAPI_LNG_OVERRIDE_NONE_1, BLPAPI_LNG_OVERRIDE_NONE_2,
    BLPAPI_LNG_OVERRIDE_NONE_3, BLPAPI_LNG_OVERRIDE_NONE_4, BLPAPI_LNG_OVERRIDE_NONE_5,
    BLPAPI_LNG_OVERRIDE_PORTUGUESE, BLPAPI_LNG_OVERRIDE_RUSSIAN, BLPAPI_LNG_OVERRIDE_SPANISH,
    BLPAPI_SECURITY_SUBTYPE_CDS, BLPAPI_SECURITY_SUBTYPE_INFLATION,
    BLPAPI_SECURITY_SUBTYPE_INVALID, BLPAPI_SECURITY_SUBTYPE_ISSUER, BLPAPI_SECURITY_SUBTYPE_OIS,
    BLPAPI_SECURITY_SUBTYPE_RATE, BLPAPI_SECURITY_SUBTYPE_SECTOR, BLPAPI_SECURITY_SUBTYPE_SENIOR,
    BLPAPI_SECURITY_SUBTYPE_SPREAD, BLPAPI_SECURITY_SUBTYPE_SUBORDINATED,
    BLPAPI_SECURITY_SUBTYPE_UNASSIGNED, BLPAPI_SECURITY_SUBTYPE_ZERO, BLPAPI_SECURITY_TYPE_AGENCY,
    BLPAPI_SECURITY_TYPE_COMDTY, BLPAPI_SECURITY_TYPE_CORP, BLPAPI_SECURITY_TYPE_CURNCY,
    BLPAPI_SECURITY_TYPE_GOVT, BLPAPI_SECURITY_TYPE_INVALID, BLPAPI_SECURITY_TYPE_IRS,
    BLPAPI_SECURITY_TYPE_MMKT, BLPAPI_SECURITY_TYPE_MTGE, BLPAPI_SECURITY_TYPE_MUNI,
    BLPAPI_SECURITY_TYPE_UNASSIGNED, BLPAPI_YELLOW_FILTER_CLNT, BLPAPI_YELLOW_FILTER_CMDT,
    BLPAPI_YELLOW_FILTER_CORP, BLPAPI_YELLOW_FILTER_CURR, BLPAPI_YELLOW_FILTER_EQTY,
    BLPAPI_YELLOW_FILTER_GOVT, BLPAPI_YELLOW_FILTER_INDX, BLPAPI_YELLOW_FILTER_MMKT,
    BLPAPI_YELLOW_FILTER_MTGE, BLPAPI_YELLOW_FILTER_MUNI, BLPAPI_YELLOW_FILTER_NONE,
    BLPAPI_YELLOW_FILTER_PRFD,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DataSeries<R> {
    pub ticker: String,
    pub eids: Vec<String>,
    pub data: R,
}

#[derive(Default, Debug)]
pub struct DataSeriesBuilder<R> {
    pub ticker: String,
    pub eids: Option<Vec<String>>,
    pub values: Vec<R>,
}

impl<R> DataSeriesBuilder<R> {
    /// Create a new timeseries with given capacity
    pub fn with_capacity(capacity: usize, ticker: String) -> Self {
        DataSeriesBuilder {
            ticker,
            eids: None,
            values: Vec::with_capacity(capacity),
        }
    }

    fn iter_entries(self, ticker: String) -> impl Iterator<Item = DataSeries<R>> {
        let eids = self.eids.unwrap_or_default();
        self.values.into_iter().map(move |data| DataSeries {
            data,
            eids: eids.clone(),
            ticker: ticker.to_string(),
        })
    }

    pub fn to_rows(self) -> Vec<DataSeries<R>> {
        let ticker = self.ticker.clone();
        self.iter_entries(ticker).collect()
    }
}

#[derive(Debug, Default)]
pub enum FieldTypes {
    Static,
    RealTime,
    #[default]
    All,
}

impl From<FieldTypes> for &str {
    fn from(v: FieldTypes) -> Self {
        match v {
            FieldTypes::Static => BLPAPI_DEFAULT_STATIC,
            FieldTypes::RealTime => BLPAPI_DEFAULT_REALTIME,
            FieldTypes::All => BLPAPI_DEFAULT_ALL,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldSeries {
    pub id: String,
    pub mnemonic: String,
    pub desc: String,
    pub data_type: Option<String>,
    pub field_type: Option<String>,
    pub field_category: Option<String>,
    pub field_documentation: Option<String>,
    pub field_property: HashMap<String, String>,
    pub field_default_formatting: HashMap<String, String>,
    pub field_error: HashMap<String, String>,
    pub other: HashMap<String, String>,
    pub overrides: Vec<String>,
}

#[derive(Default, Debug)]
pub struct FieldSeriesBuilder {
    pub id: String,
    pub mnemonic: String,
    pub desc: String,
    pub data_type: Option<String>,
    pub field_type: Option<String>,
    pub field_category: Option<String>,
    pub field_documentation: Option<String>,
    pub field_property: HashMap<String, String>,
    pub field_default_formatting: HashMap<String, String>,
    pub field_error: HashMap<String, String>,
    pub other: HashMap<String, String>,
    pub overrides: Vec<String>,
}

impl FieldSeriesBuilder {
    pub fn id(&mut self, id: String) {
        self.id = id;
    }
    pub fn mnemonic(&mut self, value: String) {
        self.mnemonic = value;
    }
    pub fn desc(&mut self, value: String) {
        self.desc = value;
    }
    pub fn data_type(&mut self, value: String) {
        self.data_type = Some(value);
    }
    pub fn field_type(&mut self, value: String) {
        self.field_type = Some(value);
    }
    pub fn field_property(&mut self, key: String, value: String) {
        self.field_property.insert(key, value);
    }
    pub fn field_category(&mut self, value: String) {
        self.field_category = Some(value);
    }
    pub fn field_default_formatting(&mut self, key: String, value: String) {
        self.field_default_formatting.insert(key, value);
    }
    pub fn field_error(&mut self, key: String, value: String) {
        self.field_error.insert(key, value);
    }
    pub fn field_documentation(&mut self, value: String) {
        self.field_documentation = Some(value);
    }
    pub fn other(&mut self, name: String, value: String) {
        self.other.insert(name, value);
    }
    pub fn overrides(&mut self, value: String) {
        self.overrides.push(value);
    }
    pub fn build(self) -> FieldSeries {
        FieldSeries {
            id: self.id,
            mnemonic: self.mnemonic,
            desc: self.desc,
            data_type: self.data_type,
            field_type: self.field_type,
            field_category: self.field_category,
            field_default_formatting: self.field_default_formatting,
            field_documentation: self.field_documentation,
            other: self.other,
            field_property: self.field_property,
            field_error: self.field_error,
            overrides: self.overrides,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum YellowKey {
    Cmdt,
    Eqty,
    Muni,
    Prfd,
    Clnt,
    Mmkt,
    Govt,
    Corp,
    Indx,
    Curr,
    Mtge,
    #[default]
    None,
}

impl From<YellowKey> for &str {
    fn from(v: YellowKey) -> Self {
        match v {
            YellowKey::Cmdt => BLPAPI_YELLOW_FILTER_CMDT,
            YellowKey::Eqty => BLPAPI_YELLOW_FILTER_EQTY,
            YellowKey::Muni => BLPAPI_YELLOW_FILTER_MUNI,
            YellowKey::Prfd => BLPAPI_YELLOW_FILTER_PRFD,
            YellowKey::Clnt => BLPAPI_YELLOW_FILTER_CLNT,
            YellowKey::Mmkt => BLPAPI_YELLOW_FILTER_MMKT,
            YellowKey::Govt => BLPAPI_YELLOW_FILTER_GOVT,
            YellowKey::Corp => BLPAPI_YELLOW_FILTER_CORP,
            YellowKey::Indx => BLPAPI_YELLOW_FILTER_INDX,
            YellowKey::Curr => BLPAPI_YELLOW_FILTER_CURR,
            YellowKey::Mtge => BLPAPI_YELLOW_FILTER_MTGE,
            YellowKey::None => BLPAPI_YELLOW_FILTER_NONE,
        }
    }
}

impl TryFrom<&str> for YellowKey {
    type Error = String;

    fn try_from(v: &str) -> Result<YellowKey, Self::Error> {
        match v {
            BLPAPI_YELLOW_FILTER_CMDT => Ok(YellowKey::Cmdt),
            BLPAPI_YELLOW_FILTER_EQTY => Ok(YellowKey::Eqty),
            BLPAPI_YELLOW_FILTER_MUNI => Ok(YellowKey::Muni),
            BLPAPI_YELLOW_FILTER_PRFD => Ok(YellowKey::Prfd),
            BLPAPI_YELLOW_FILTER_CLNT => Ok(YellowKey::Clnt),
            BLPAPI_YELLOW_FILTER_MMKT => Ok(YellowKey::Mmkt),
            BLPAPI_YELLOW_FILTER_GOVT => Ok(YellowKey::Govt),
            BLPAPI_YELLOW_FILTER_CORP => Ok(YellowKey::Corp),
            BLPAPI_YELLOW_FILTER_INDX => Ok(YellowKey::Indx),
            BLPAPI_YELLOW_FILTER_CURR => Ok(YellowKey::Curr),
            BLPAPI_YELLOW_FILTER_MTGE => Ok(YellowKey::Mtge),
            BLPAPI_YELLOW_FILTER_NONE => Ok(YellowKey::None),
            _ => Err(format!("Unkown Key")),
        }
    }
}

#[derive(Debug, Default)]
pub enum Language {
    English,
    Kanji,
    French,
    German,
    Spanish,
    Portuguese,
    Italien,
    ChineseTrad,
    ChineseSimp,
    Korean,
    None1,
    None2,
    None3,
    None4,
    None5,
    Russian,
    #[default]
    None,
}

impl From<Language> for &str {
    fn from(v: Language) -> Self {
        match v {
            Language::English => BLPAPI_LNG_OVERRIDE_ENGLISH,
            Language::Kanji => BLPAPI_LNG_OVERRIDE_KANJI,
            Language::French => BLPAPI_LNG_OVERRIDE_FRENCH,
            Language::German => BLPAPI_LNG_OVERRIDE_GERMAN,
            Language::Spanish => BLPAPI_LNG_OVERRIDE_SPANISH,
            Language::Portuguese => BLPAPI_LNG_OVERRIDE_PORTUGUESE,
            Language::Italien => BLPAPI_LNG_OVERRIDE_ITALIAN,
            Language::ChineseTrad => BLPAPI_LNG_OVERRIDE_CHINESE_TRAD,
            Language::ChineseSimp => BLPAPI_LNG_OVERRIDE_CHINESE_SIMP,
            Language::Korean => BLPAPI_LNG_OVERRIDE_KOREAN,
            Language::None1 => BLPAPI_LNG_OVERRIDE_NONE_1,
            Language::None2 => BLPAPI_LNG_OVERRIDE_NONE_2,
            Language::None3 => BLPAPI_LNG_OVERRIDE_NONE_3,
            Language::None4 => BLPAPI_LNG_OVERRIDE_NONE_4,
            Language::None5 => BLPAPI_LNG_OVERRIDE_NONE_5,
            Language::Russian => BLPAPI_LNG_OVERRIDE_RUSSIAN,
            Language::None => BLPAPI_LNG_OVERRIDE_NONE,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Security {
    pub id: String,
    pub yellow_key: YellowKey,
    pub security: String,
    pub parse_key: String,
    pub ticker: String,
    pub country_code: Option<String>,
    pub name: Option<String>,
    pub market_sector: Option<String>,
    pub instrument_type: Option<String>,
    pub description: Option<String>,
    pub currency: Option<String>,
    pub curve_id: Option<String>,
    pub security_type: Option<String>,
    pub security_subtype: Option<String>,
    pub publisher: Option<String>,
    pub bbg_id: Option<String>,
    pub isin: Option<String>,
    pub sedol: Option<String>,
}

#[derive(Default, Debug)]
pub struct SecurityBuilder {
    pub id: String,
    pub yellow_key: YellowKey,
    pub security: String,
    pub parse_key: String,
    pub ticker: String,
    pub country_code: Option<String>,
    pub name: Option<String>,
    pub market_sector: Option<String>,
    pub instrument_type: Option<String>,
    pub description: Option<String>,
    pub currency: Option<String>,
    pub curve_id: Option<String>,
    pub security_type: Option<String>,
    pub security_subtype: Option<String>,
    pub publisher: Option<String>,
    pub bbg_id: Option<String>,
    pub isin: Option<String>,
    pub sedol: Option<String>,
}

impl SecurityBuilder {
    pub fn description(&mut self, value: String) {
        self.description = Some(value);
    }
    pub fn currency(&mut self, value: String) {
        self.currency = Some(value);
    }
    pub fn curve_id(&mut self, value: String) {
        self.curve_id = Some(value);
    }
    pub fn security_type(&mut self, value: String) {
        self.security_type = Some(value);
    }
    pub fn security_subtype(&mut self, value: String) {
        self.security_subtype = Some(value);
    }
    pub fn publisher(&mut self, value: String) {
        self.publisher = Some(value);
    }
    pub fn bbg_id(&mut self, value: String) {
        self.bbg_id = Some(value);
    }
    pub fn name(&mut self, value: String) {
        self.name = Some(value);
    }
    pub fn isin(&mut self, value: String) {
        self.isin = Some(value);
    }
    pub fn sedol(&mut self, value: String) {
        self.sedol = Some(value);
    }
    pub fn id(&mut self, id: String) {
        self.id = id;
    }
    pub fn yellow_key(&mut self, value: &str) {
        let yk = YellowKey::try_from(value).unwrap_or_default();
        self.yellow_key = yk;
    }
    pub fn security(&mut self, value: String) {
        self.security = value;
    }
    pub fn parse_key(&mut self, value: String) {
        self.parse_key = value;
    }
    pub fn ticker(&mut self, value: String) {
        self.ticker = value;
    }
    pub fn country_code(&mut self, value: String) {
        self.country_code = Some(value);
    }
    pub fn market_sector(&mut self, value: String) {
        self.market_sector = Some(value);
    }
    pub fn instrument_type(&mut self, value: String) {
        self.instrument_type = Some(value);
    }
    pub fn build(self) -> Security {
        Security {
            id: self.id,
            name: self.name,
            yellow_key: self.yellow_key,
            security: self.security,
            parse_key: self.parse_key,
            ticker: self.ticker,
            country_code: self.country_code,
            market_sector: self.market_sector,
            instrument_type: self.instrument_type,
            description: self.description,
            currency: self.currency,
            curve_id: self.curve_id,
            security_type: self.security_type,
            security_subtype: self.security_subtype,
            publisher: self.publisher,
            bbg_id: self.bbg_id,
            isin: self.isin,
            sedol: self.sedol,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SecurityLookUp {
    pub query: String,
    pub total_results: i32,
    pub results: Vec<Security>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SecurityLookUpBuilder {
    pub query: String,
    pub total_results: i32,
    pub results: Vec<Security>,
}

impl SecurityLookUpBuilder {
    pub fn query(&mut self, query: String) {
        self.query = query;
    }
    pub fn total_results(&mut self, results: i32) {
        self.total_results = results;
    }
    pub fn results(&mut self, results: Vec<Security>) {
        self.results = results;
    }
    pub fn build(self) -> SecurityLookUp {
        SecurityLookUp {
            query: self.query,
            total_results: self.total_results,
            results: self.results,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum SecurityType {
    Invalid,
    Irs,
    Govt,
    Agency,
    Muni,
    Corp,
    Mtge,
    Mmkt,
    Curncy,
    Comdty,
    #[default]
    Unassigned,
}

impl From<SecurityType> for &str {
    fn from(v: SecurityType) -> Self {
        match v {
            SecurityType::Invalid => BLPAPI_SECURITY_TYPE_INVALID,
            SecurityType::Irs => BLPAPI_SECURITY_TYPE_IRS,
            SecurityType::Govt => BLPAPI_SECURITY_TYPE_GOVT,
            SecurityType::Agency => BLPAPI_SECURITY_TYPE_AGENCY,
            SecurityType::Muni => BLPAPI_SECURITY_TYPE_MUNI,
            SecurityType::Corp => BLPAPI_SECURITY_TYPE_CORP,
            SecurityType::Mtge => BLPAPI_SECURITY_TYPE_MTGE,
            SecurityType::Mmkt => BLPAPI_SECURITY_TYPE_MMKT,
            SecurityType::Curncy => BLPAPI_SECURITY_TYPE_CURNCY,
            SecurityType::Comdty => BLPAPI_SECURITY_TYPE_COMDTY,
            SecurityType::Unassigned => BLPAPI_SECURITY_TYPE_UNASSIGNED,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum SecuritySubType {
    Invalid,
    Senior,
    SubOrdinated,
    Zero,
    Ois,
    Inflation,
    Spread,
    Cds,
    Rate,
    Sector,
    Issuer,
    #[default]
    Unassigned,
}
impl From<SecuritySubType> for &str {
    fn from(v: SecuritySubType) -> Self {
        match v {
            SecuritySubType::Invalid => BLPAPI_SECURITY_SUBTYPE_INVALID,
            SecuritySubType::Senior => BLPAPI_SECURITY_SUBTYPE_SENIOR,
            SecuritySubType::SubOrdinated => BLPAPI_SECURITY_SUBTYPE_SUBORDINATED,
            SecuritySubType::Zero => BLPAPI_SECURITY_SUBTYPE_ZERO,
            SecuritySubType::Ois => BLPAPI_SECURITY_SUBTYPE_OIS,
            SecuritySubType::Inflation => BLPAPI_SECURITY_SUBTYPE_INFLATION,
            SecuritySubType::Spread => BLPAPI_SECURITY_SUBTYPE_SPREAD,
            SecuritySubType::Cds => BLPAPI_SECURITY_SUBTYPE_CDS,
            SecuritySubType::Rate => BLPAPI_SECURITY_SUBTYPE_RATE,
            SecuritySubType::Sector => BLPAPI_SECURITY_SUBTYPE_SECTOR,
            SecuritySubType::Issuer => BLPAPI_SECURITY_SUBTYPE_ISSUER,
            SecuritySubType::Unassigned => BLPAPI_SECURITY_SUBTYPE_UNASSIGNED,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CurveOptions {
    pub query: String,
    pub bbg_id: Option<String>,
    pub country: Option<String>,
    pub currency: Option<String>,
    pub curve_id: Option<String>,
    pub sec_type: Option<SecurityType>,
    pub sec_subtype: Option<SecuritySubType>,
}

impl CurveOptions {
    pub fn new<S: Into<String>>(query: S) -> Self {
        let query = query.into();
        Self {
            query,
            ..Self::default()
        }
    }

    pub fn bbg_id<S: Into<String>>(mut self, id: S) -> Self {
        let id = id.into();
        self.bbg_id = Some(id);
        self
    }
    pub fn country<S: Into<String>>(mut self, id: S) -> Self {
        let id = id.into();
        self.country = Some(id);
        self
    }
    pub fn currency<S: Into<String>>(mut self, id: S) -> Self {
        let id = id.into();
        self.currency = Some(id);
        self
    }
    pub fn curve_id<S: Into<String>>(mut self, id: S) -> Self {
        let id = id.into();
        self.curve_id = Some(id);
        self
    }
    pub fn security_type(mut self, id: SecurityType) -> Self {
        self.sec_type = Some(id);
        self
    }
    pub fn security_subtype(mut self, id: SecuritySubType) -> Self {
        self.sec_subtype = Some(id);
        self
    }
}
