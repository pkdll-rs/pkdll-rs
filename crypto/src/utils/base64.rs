use base64::{
    alphabet,
    engine::{
        general_purpose::{NO_PAD, STANDARD_NO_PAD as BASE64},
        GeneralPurpose,
    },
    DecodeError, Engine,
};

const BCRYPT: GeneralPurpose = GeneralPurpose::new(&alphabet::BCRYPT, NO_PAD);

pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
    BASE64.encode(input)
}

pub fn decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, DecodeError> {
    BASE64.decode(input)
}

pub fn decode_bcrypt<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, DecodeError> {
    BCRYPT.decode(input)
}
