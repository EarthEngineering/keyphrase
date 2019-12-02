use crate::crypto::pbkdf2;
use crate::keyphrase::KeyPhrase;
use std::fmt;

/// The secret value used to derive HD wallet addresses from a [`KeyPhrase`][KeyPhrase] phrase.
///
/// Because it is not possible to create a [`KeyPhrase`][KeyPhrase] instance that is invalid, it is
/// therefore impossible to have a [`Seed`][Seed] instance that is invalid. This guarantees that only
/// a valid, intact mnemonic phrase can be used to derive HD wallet addresses.
///
/// To get the raw byte value use [`Seed::as_bytes()`][Seed::as_bytes()]. These can be used to derive
/// HD wallet addresses using another crate (deriving HD wallet addresses is outside the scope of this
/// crate and the BIP39 standard).
///
/// [KeyPhrase]: ./keyphrase/struct.KeyPhrase.html
/// [Seed]: ./seed/struct.Seed.html
/// [Seed::as_bytes()]: ./seed/struct.Seed.html#method.as_bytes

#[derive(Clone)]
pub struct Seed {
    bytes: Vec<u8>,
}

impl Seed {
    /// Generates the seed from the [`KeyPhrase`][KeyPhrase] and the password.
    ///
    /// [KeyPhrase]: ./keyphrase/struct.KeyPhrase.html
    pub fn new(keyphrase: &KeyPhrase, password: &str) -> Self {
        let salt = format!("keyphrase{}", password);
        let bytes = pbkdf2(keyphrase.entropy(), &salt);

        Self { bytes }
    }

    /// Get the seed value as a byte slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl AsRef<[u8]> for Seed {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl fmt::Debug for Seed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#X}", self)
    }
}

impl fmt::LowerHex for Seed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }

        for byte in &self.bytes {
            write!(f, "{:x}", byte)?;
        }

        Ok(())
    }
}

impl fmt::UpperHex for Seed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }

        for byte in &self.bytes {
            write!(f, "{:X}", byte)?;
        }

        Ok(())
    }
}
