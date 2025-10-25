use anyhow::Result;
use pallas::codec::minicbor::Encode;
use pallas::codec::minicbor::Encoder;
use pallas::crypto::hash::Hash;
use pallas::ledger::primitives::conway::{BoundedBytes, Constr, MaybeIndefArray, PlutusData};
use pallas::txbuilder::Input;
use std::str::FromStr;

pub fn string_to_input_pallas(input_str: &str) -> Result<Input, String> {
    let parts: Vec<&str> = input_str.split('#').collect();
    if parts.len() != 2 {
        return Err("Invalid format, expected 'hash#int'".into());
    }

    let hash = Hash::from_str(parts[0]).map_err(|e| e.to_string())?;
    let index = parts[1]
        .parse::<u64>()
        .map_err(|_| "Invalid integer part".to_string())?;

    Ok(Input::new(hash, index))
}

pub fn encode_plutus_data(datum: PlutusData) -> Vec<u8> {
    let mut buf = Vec::new();
    let mut enc = Encoder::new(&mut buf);
    datum.encode(&mut enc, &mut ()).unwrap();
    buf
}

pub fn hex_to_array32(s: &str) -> Result<[u8; 32], String> {
    let bytes = hex::decode(s).map_err(|e| e.to_string())?;
    if bytes.len() != 32 {
        return Err(format!("expected 32 bytes, got {}", bytes.len()));
    }
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&bytes);
    Ok(arr)
}

// TODO: this does not seem to be the correct way to build datum
pub fn string_to_datum(s: &str) -> Result<Vec<u8>> {
    let datum = PlutusData::Constr(Constr {
        tag: 121,
        any_constructor: None,
        fields: MaybeIndefArray::Def(vec![PlutusData::BoundedBytes(BoundedBytes::from(
            s.as_bytes().to_vec(),
        ))]),
    });

    Ok(encode_plutus_data(datum))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_datum() {
        let input = "hello";
        let result = string_to_datum(input).unwrap();

        let expected_hex = "d879814568656c6c6f";
        let expected_bytes = hex::decode(expected_hex).unwrap();

        assert_eq!(result, expected_bytes);
    }
}
