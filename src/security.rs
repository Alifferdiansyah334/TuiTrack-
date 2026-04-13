use anyhow::{Context, Result};
use sha2::{Digest, Sha256};

pub fn hash_passkey(salt: &str, passkey: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(salt.as_bytes());
    hasher.update(b":");
    hasher.update(passkey.as_bytes());
    hex_encode(&hasher.finalize())
}

pub fn encrypt_note(passkey: &str, salt: &str, content: &str) -> String {
    let encrypted = xor_keystream(content.as_bytes(), passkey.as_bytes(), salt.as_bytes());
    hex_encode(&encrypted)
}

pub fn decrypt_note(passkey: &str, salt: &str, encrypted_hex: &str) -> Result<String> {
    let encrypted = hex_decode(encrypted_hex)?;
    let decrypted = xor_keystream(&encrypted, passkey.as_bytes(), salt.as_bytes());
    String::from_utf8(decrypted).context("konten note terenkripsi tidak valid")
}

fn xor_keystream(input: &[u8], passkey: &[u8], salt: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len());
    let mut counter = 0u64;

    while output.len() < input.len() {
        let mut hasher = Sha256::new();
        hasher.update(passkey);
        hasher.update(b":");
        hasher.update(salt);
        hasher.update(counter.to_be_bytes());
        let block = hasher.finalize();
        for byte in block {
            if output.len() == input.len() {
                break;
            }
            let idx = output.len();
            output.push(input[idx] ^ byte);
        }
        counter = counter.saturating_add(1);
    }

    output
}

fn hex_encode(bytes: &[u8]) -> String {
    const TABLE: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(TABLE[(byte >> 4) as usize] as char);
        out.push(TABLE[(byte & 0x0f) as usize] as char);
    }
    out
}

fn hex_decode(value: &str) -> Result<Vec<u8>> {
    let bytes = value.as_bytes();
    if !bytes.len().is_multiple_of(2) {
        anyhow::bail!("format note terenkripsi tidak valid");
    }

    bytes
        .chunks(2)
        .map(|chunk| {
            let high = decode_nibble(chunk[0])?;
            let low = decode_nibble(chunk[1])?;
            Ok((high << 4) | low)
        })
        .collect()
}

fn decode_nibble(byte: u8) -> Result<u8> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => anyhow::bail!("format note terenkripsi tidak valid"),
    }
}
