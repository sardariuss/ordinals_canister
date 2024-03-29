use candid::{CandidType, Deserialize};

use ic_cdk::api::call::RejectionCode;

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct HttpSendError {
    pub rejection_code: RejectionCode,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub enum OrdError {
    HttpSendError(HttpSendError),
    ResponseError(String),
    ResponseEncodingError(String),
    ResponseDecodingError(String),
    ContextEncodingError(String),
    ContextDecodingError(String),
    NoServiceError { 
        providers: Vec<Provider>, 
        end_point: EndPoint 
    },
    TooFewCycles {
        expected: u128,
        received: u128,
    },
    UnexpectedResponseTypeError(Response),
}

pub type OrdResult = Result<Response, OrdError>;

#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct ProviderOrdResult {
    pub provider: Provider,
    pub result: OrdResult,
}

#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum MultiOrdResult {
    Consistent(OrdResult),
    Inconsistent(Vec<ProviderOrdResult>),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, CandidType, Deserialize, Copy, Clone, Hash)]
pub enum Provider {
    Hiro,
    Bitgem,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct OrdArgs {
    pub function: OrdFunction,
    pub providers: Vec<Provider>,
    pub max_kb_per_item: Option<u64>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Args {
    pub function: OrdFunction,
    pub max_kb_per_item: Option<u64>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum OrdFunction {
    SatRange(SatRangeArgs),
    SatInfo(SatInfoArgs),
    SatInscriptions(SatInscriptionsArgs),
    InscriptionInfo(InscriptionInfoArgs),
    InscriptionContent(InscriptionContentArgs),
    Brc20Details(Brc20DetailsArgs),
    Brc20Holders(Brc20HoldersArgs),
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct SatRangeArgs { 
    pub utxos: Vec<Utxo>,
    pub exclude_common_ranges: bool,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct SatInfoArgs {
    pub ordinal: u64
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct SatInscriptionsArgs {
    pub ordinal: u64,
    pub offset: u64,
    pub limit: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InscriptionInfoArgs {
    pub inscription_id: String
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InscriptionContentArgs {
    pub inscription_id: String
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Brc20DetailsArgs {
    pub ticker: String
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Brc20HoldersArgs {
    pub ticker: String,
    pub offset: u64,
    pub limit: u64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, CandidType, Deserialize, Copy, Clone)]
pub enum EndPoint {
    SatRange,
    SatInfo,
    SatInscriptions,
    InscriptionInfo,
    InscriptionContent,
    Brc20Details,
    Brc20Holders,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub enum Response {
    SatRange(SatRanges),
    SatInfo(SatInfo),
    SatInscriptions(HiroSatInscriptions),
    InscriptionInfo(HiroSatInscription),
    InscriptionContent(InscriptionContent),
    Brc20Details(HiroBrc20Details),
    Brc20Holders(HiroBrc20Holders)
}


#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct Utxo {
    pub txid: String,
    pub vout: u32,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct SatRanges {
    pub ranges: Option<Vec<SatRange>>,
    pub exotic_ranges : Option<Vec<ExoticSatRange>>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
#[allow(non_snake_case)]
pub struct BitgemSatRanges {
    pub ranges : Option<Vec<SatRange>>,
    pub exoticRanges : Option<Vec<BitgemExoticSatRange>>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct SatRange {
    pub utxo : String,
    pub start : u64,
    pub size : u64,
    pub end : u64,
    pub offset : u64,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct ExoticSatRange {
    pub utxo : String,
    pub start : u64,
    pub size : u64,
    pub end : u64,
    pub offset : u64,
    pub rarity : SatoshiRarity,
    pub satributes : Vec<String>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct BitgemExoticSatRange {
    pub utxo : String,
    pub start : u64,
    pub size : u64,
    pub end : u64,
    pub offset : u64,
    pub satributes : Vec<String>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct HiroSatInfo {
    pub coinbase_height: u64,
    pub cycle: u64,
    pub decimal: String,
    pub degree: String,
    pub inscription_id: Option<String>,
    pub epoch: u64,
    pub name: String,
    pub offset: u64,
    pub percentile: String,
    pub period: u64,
    pub rarity: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct BitgemSatInfo {
    pub sat: u64,
    pub height: u64,
    pub cycle: u64,
    pub epoch: u64,
    pub period: u64,
    pub satributes: Vec<String>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub enum SatoshiRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

// Common ordinal info struct, contains the fiels that are common to all providers
#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct SatInfo {
    pub height: u64,
    pub cycle: u64,
    pub epoch: u64,
    pub period: u64,
    pub rarity: SatoshiRarity,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HiroInscriptionContentArgs {
    pub inscription_id: String,
    pub max_content_kb: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct HiroSatInscription {
    pub id: String,
    pub number: i64,
    pub address: String,
    pub genesis_address: String,
    pub genesis_block_height: u64,
    pub genesis_block_hash: String,
    pub genesis_tx_id: String,
    pub genesis_fee: String,
    pub genesis_timestamp: u64,
    pub tx_id: String,
    pub location: String,
    pub output: String,
    pub value: String,
    pub offset: String,
    pub sat_ordinal: String,
    pub sat_rarity: String,
    pub sat_coinbase_height: u64,
    pub mime_type: String,
    pub content_type: String,
    pub content_length: u64,
    pub timestamp: u64,
    pub curse_type: Option<String>,
    pub recursive: bool,
    pub recursion_refs: Option<Vec<String>>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct HiroSatInscriptions {
    pub limit: u64,
    pub offset: u64,
    pub total: u64,
    pub results: Vec<HiroSatInscription>,
}

pub type InscriptionContent = Vec<u8>;

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct HiroBrc20Details {
    pub token: HiroBrc20Token,
    pub supply: HiroBrc20Supply,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct HiroBrc20Token {
    pub id: String,
    pub number: u64,
    pub block_height: u64,
    pub tx_id: String,
    pub address: String,
    pub ticker: String,
    pub max_supply: String,
    pub mint_limit: String,
    pub decimals: u64,
    pub deploy_timestamp: u64,
    pub minted_supply: String,
    pub tx_count: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct HiroBrc20Supply {
    pub max_supply: String,
    pub minted_supply: String,
    pub holders: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct HiroBrc20Holders {
    pub limit: u64,
    pub offset: u64,
    pub total: u64,
    pub results: Vec<HiroBrc20Holder>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct HiroBrc20Holder {
    pub address: String,
    pub overall_balance: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HiroBrc20HoldersArgs {
    pub ticker: String,
    pub limit: u64,
    pub offset: u64,
}

#[derive(Deserialize)]
pub struct JsonError {
    pub error: String,
}