use candid::{CandidType, Deserialize};

use ic_cdk::api::call::RejectionCode;

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct HttpSendError {
    pub rejection_code: RejectionCode,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub enum OrdError {
    HttpSendError(HttpSendError),
    CandidEncodingError(String),
    CandidDecodingError(String),
    NoServiceError{ provider: Provider, end_point: EndPoint },
    UnexpectedResponseTypeError(Response),
}

pub type OrdResult = Result<Response, OrdError>;

#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum MultiOrdResult {
    Consistent(OrdResult),
    Inconsistent(Vec<(Provider, OrdResult)>),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, CandidType, Deserialize, Copy, Clone)]
pub enum Provider {
    Hiro,
    Bitgem,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Args {
    pub function: Function,
    pub query_options: Option<QueryOptions>,
    pub max_kb_per_item: Option<u64>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum Function {
    SatRange { 
        utxo: Utxo,
    },
    SatInfo {
        ordinal: u64,
    },
    SatInscriptions {
        ordinal: u64,
    },
    InscriptionInfo {
        inscription_id: String,
    },
    InscriptionContent {
        inscription_id: String,
    },
    Brc20Details {
        ticker: String,
    },
    Brc20Holders {
        ticker: String,
    },
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct QueryOptions {
    pub limit: u64,
    pub offset: u64,
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
    SatRange(BitgemSatRanges),
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
#[allow(non_snake_case)]
pub struct BitgemSatRanges {
    pub ranges : Vec<BitgemSatRange>,
    pub exoticRanges : Vec<BitgemExoticSatRange>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct BitgemSatRange {
    pub utxo : String,
    pub start : u64,
    pub size : u64,
    pub end : u64,
    pub offset : u64,
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
    pub coinbase_height: u64,           // 8 bytes
    pub cycle: u64,                     // 8 bytes
    pub decimal: String,                // assumed max 32 bytes, like "51483.3248345364"
    pub degree: String,                 // assumed max 64 bytes, like "0°51483′1083″3248345364‴"
    pub inscription_id: Option<String>, // assumed max 128 bytes, like "38c46a8bf7ec90bc7f6b797e7dc84baa97f4e5fd4286b92fe1b50176d03b18dci0"
    pub epoch: u64,                     // 8 bytes
    pub name: String,                   // assumed max 128 bytes
    pub offset: u64,                    // 8 bytes
    pub percentile: String,             // assumed max 32 bytes "12.258011839453534%"
    pub period: u64,                    // 8 bytes
    pub rarity: String,                 // assumed max 64 bytes, like "0°51483′1083″3248345364‴"
}
// total bytes: 8 + 8 + 32 + 64 + 128 + 8 + 32 + 8 + 64 = 352 bytes

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct BitgemSatInfo {
    pub sat: u64,
    pub height: u64,
    pub cycle: u64,
    pub epoch: u64,
    pub period: u64,
    pub satributes: Vec<String>,
}

// Satoshi rarity
#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub enum SatoshiRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

// Common ordinal info struct, contains the fiels that are common to both (intersecting)
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
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HiroSatInscriptionsArgs {
    pub ordinal: u64,
    pub limit: u64,
    pub offset: u64,
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