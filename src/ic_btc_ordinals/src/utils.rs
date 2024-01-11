// @todo

//use candid::{decode_args, encode_args, Deserialize, Error, CandidType};

///// Checks if the canister is supporting IPv4 exchanges and forex sources.
//pub(crate) fn is_ipv4_support_available() -> bool {
//  cfg!(feature = "ipv4-support")
//}
//
//// @todo: useless ?
//pub fn candid_encode<T>(response: T) -> Result<Vec<u8>, Error> 
//where
//    T: CandidType
//{
//    encode_args((response,))
//}
//
//// @todo: useless ?
//pub fn candid_decode<'a, T>(bytes: &'a [u8]) -> Result<T, Error> 
//where 
//    T: CandidType + Deserialize<'a>
//{
//    decode_args::<(T,)>(bytes).map(|decoded| decoded.0)
//}