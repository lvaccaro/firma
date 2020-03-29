use crate::ErrorJson;
use serde_json::Value;

#[derive(Debug)]
pub enum Error {
    Generic(String),

    // Internal
    AlreadySigned,

    // External
    BitcoinRpc(bitcoincore_rpc::Error),
    BitcoinEncode(bitcoin::consensus::encode::Error),
    BitcoinKey(bitcoin::util::key::Error),
    BitcoinSecp256k1(bitcoin::secp256k1::Error),
    BitcoinPSBT(bitcoin::util::psbt::Error),
    BitcoinAddress(bitcoin::util::address::Error),
    BitcoinBech32(bitcoin::bech32::Error),
    Serde(serde_json::error::Error),
    IO(std::io::Error),
    Base58(bitcoin::util::base58::Error),
    Bip32(bitcoin::util::bip32::Error),
    Base64(base64::DecodeError),
    PathStrip(std::path::StripPrefixError),
    Qr(qrcode::types::QrError),
    Hex(hex::FromHexError),
    Env(std::env::VarError),
}

macro_rules! impl_error {
    ( $from:ty, $to:ident ) => {
        impl std::convert::From<$from> for Error {
            fn from(err: $from) -> Self {
                Error::$to(err)
            }
        }
    };
}

impl_error!(bitcoincore_rpc::Error, BitcoinRpc);
impl_error!(bitcoin::util::base58::Error, Base58);
impl_error!(bitcoin::util::bip32::Error, Bip32);
impl_error!(bitcoin::consensus::encode::Error, BitcoinEncode);
impl_error!(bitcoin::util::key::Error, BitcoinKey);
impl_error!(bitcoin::secp256k1::Error, BitcoinSecp256k1);
impl_error!(bitcoin::util::psbt::Error, BitcoinPSBT);
impl_error!(bitcoin::util::address::Error, BitcoinAddress);
impl_error!(bitcoin::bech32::Error, BitcoinBech32);
impl_error!(serde_json::error::Error, Serde);
impl_error!(std::io::Error, IO);
impl_error!(base64::DecodeError, Base64);
impl_error!(std::path::StripPrefixError, PathStrip);
impl_error!(qrcode::types::QrError, Qr);
impl_error!(hex::FromHexError, Hex);
impl_error!(std::env::VarError, Env);

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::Generic(e) => e.to_string(),

            Error::AlreadySigned => "The json psbt already contain signed_psbt or only_sigs, exiting to avoid risk of overwriting data".to_string(),

            Error::BitcoinRpc(e) => e.to_string(),
            Error::Serde(e) => e.to_string(),
            Error::IO(e) => e.to_string(),
            Error::Base58(e) => e.to_string(),
            Error::Bip32(e) => e.to_string(),
            Error::Base64(e) => e.to_string(),
            Error::BitcoinEncode(e) => e.to_string(),
            Error::BitcoinKey(e) => e.to_string(),
            Error::BitcoinSecp256k1(e) => e.to_string(),
            Error::BitcoinPSBT(e) => e.to_string(),
            Error::BitcoinAddress(e) => e.to_string(),
            Error::BitcoinBech32(e) => e.to_string(),
            Error::PathStrip(e) => e.to_string(),
            Error::Qr(e) => e.to_string(),
            Error::Hex(e) => e.to_string(),
            Error::Env(e) => e.to_string(),

        }
    }
}

pub fn err<R>(str: &str) -> Result<R, Error> {
    Err(Error::Generic(str.into()))
}

pub fn fn_err(str: &str) -> impl Fn() -> Error + '_ {
    move || Error::Generic(str.into())
}

pub fn io_err(str: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidInput, str.to_string())
}

impl From<String> for Error {
    fn from(e: String) -> Error {
        Error::Generic(e)
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Error {
        Error::Generic(e.to_string())
    }
}

impl Error {
    pub fn to_json(self) -> Result<Value, Error> {
        let value = ErrorJson {
            error: self.to_string(),
        };
        Ok(serde_json::to_value(&value)?)
    }
}
