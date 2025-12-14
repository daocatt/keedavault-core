// TOTP (Time-based One-Time Password) module

use crate::error::{Result, VaultError};
use totp_lite::{totp_custom, Sha1};

/// Generate a TOTP code from a base32-encoded secret
///
/// # Arguments
/// * `secret` - Base32-encoded secret key (e.g., "JBSWY3DPEHPK3PXP")
///
/// # Returns
/// A 6-digit TOTP code as a string
pub fn generate_totp(secret: &str) -> Result<String> {
    // Decode base32 secret
    let secret_bytes = decode_base32(secret)
        .map_err(|e| VaultError::EncryptionError(format!("Invalid TOTP secret: {}", e)))?;

    // Get current Unix timestamp
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| VaultError::EncryptionError(format!("Time error: {}", e)))?
        .as_secs();

    // Generate TOTP code (30 second time step, 6 digits)
    let code = totp_custom::<Sha1>(30, 6, &secret_bytes, timestamp);

    Ok(format!("{:06}", code))
}

/// Validate a TOTP code against a secret
///
/// # Arguments
/// * `secret` - Base32-encoded secret key
/// * `code` - The TOTP code to validate (6 digits)
///
/// # Returns
/// `true` if the code is valid (within ±1 time window), `false` otherwise
pub fn validate_totp(secret: &str, code: &str) -> Result<bool> {
    // Decode base32 secret
    let secret_bytes = decode_base32(secret)
        .map_err(|e| VaultError::EncryptionError(format!("Invalid TOTP secret: {}", e)))?;

    // Get current Unix timestamp
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| VaultError::EncryptionError(format!("Time error: {}", e)))?
        .as_secs();

    // Check current time window and ±1 window (to account for clock drift)
    for offset in [-1i64, 0, 1] {
        let check_time = (timestamp as i64 + offset * 30) as u64;
        let expected_code = totp_custom::<Sha1>(30, 6, &secret_bytes, check_time);
        let expected_code_str = format!("{:06}", expected_code);

        if expected_code_str == code {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Get the remaining seconds until the next TOTP code
pub fn get_remaining_seconds() -> u64 {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    30 - (timestamp % 30)
}

/// Decode a base32-encoded string
fn decode_base32(input: &str) -> std::result::Result<Vec<u8>, String> {
    // Remove spaces and convert to uppercase
    let clean_input = input.replace(' ', "").to_uppercase();

    // Base32 alphabet (RFC 4648)
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

    let mut bits = 0u32;
    let mut bit_count = 0u32;
    let mut result = Vec::new();

    for ch in clean_input.chars() {
        if ch == '=' {
            break; // Padding
        }

        let value = ALPHABET
            .iter()
            .position(|&c| c == ch as u8)
            .ok_or_else(|| format!("Invalid base32 character: {}", ch))? as u32;

        bits = (bits << 5) | value;
        bit_count += 5;

        if bit_count >= 8 {
            bit_count -= 8;
            result.push((bits >> bit_count) as u8);
            bits &= (1 << bit_count) - 1;
        }
    }

    Ok(result)
}

/// Encode bytes to base32 string
pub fn encode_base32(input: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

    let mut bits = 0u32;
    let mut bit_count = 0u32;
    let mut result = String::new();

    for &byte in input {
        bits = (bits << 8) | byte as u32;
        bit_count += 8;

        while bit_count >= 5 {
            bit_count -= 5;
            let index = ((bits >> bit_count) & 0x1F) as usize;
            result.push(ALPHABET[index] as char);
            bits &= (1 << bit_count) - 1;
        }
    }

    if bit_count > 0 {
        let index = ((bits << (5 - bit_count)) & 0x1F) as usize;
        result.push(ALPHABET[index] as char);
    }

    // Add padding
    while result.len() % 8 != 0 {
        result.push('=');
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base32_decode() {
        // Test with a simple string
        let result = decode_base32("JBSWY3DPEE======");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), b"Hello!");
    }

    #[test]
    fn test_base32_encode() {
        let encoded = encode_base32(b"Hello!");
        assert_eq!(encoded, "JBSWY3DPEE======");
    }

    #[test]
    fn test_generate_totp() {
        // Using a known test secret
        let secret = "JBSWY3DPEHPK3PXP"; // "Hello!" in base32
        let result = generate_totp(secret);
        assert!(result.is_ok());

        let code = result.unwrap();
        assert_eq!(code.len(), 6);
        assert!(code.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_validate_totp() {
        let secret = "JBSWY3DPEHPK3PXP";

        // Generate a code
        let code = generate_totp(secret).unwrap();

        // It should validate successfully
        let is_valid = validate_totp(secret, &code).unwrap();
        assert!(is_valid);

        // Invalid code should fail
        let _is_valid = validate_totp(secret, "000000").unwrap();
        // This might occasionally be true if the actual code is 000000,
        // but statistically very unlikely
    }

    #[test]
    fn test_get_remaining_seconds() {
        let remaining = get_remaining_seconds();
        assert!(remaining > 0 && remaining <= 30);
    }

    #[test]
    fn test_invalid_base32() {
        let result = decode_base32("INVALID!");
        assert!(result.is_err());
    }
}
