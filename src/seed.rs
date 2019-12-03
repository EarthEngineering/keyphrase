use crate::crypto::pbkdf2;
use crate::keyphrase::KeyPhrase;
use std::fmt;

/// The secret value used to derive HD wallet addresses from a [`KeyPhrase`][KeyPhrase] phrase.
///
/// Because it is not possible to create a [`KeyPhrase`][KeyPhrase] instance that is invalid, it is
/// therefore impossible to have a [`Seed`][Seed] instance that is invalid. This guarantees that only
/// a valid, intact KeyPhrase can be used to derive HD wallet addresses.
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
    ///
    /// # Example
    ///
    /// ```
    /// use keyphrase::{KeyPhrase, KeyPhraseType, Language, Seed};
    ///
    /// let word_count: KeyPhraseType = KeyPhraseType::Words15;
    ///
    /// let lang: Language = Language::English;
    ///
    /// let keyphrase: KeyPhrase = KeyPhrase::new(word_count, lang);
    ///
    /// let seed: Seed = Seed::new(&keyphrase, "");
    /// ```
    pub fn new(keyphrase: &KeyPhrase, password: &str) -> Self {
        let salt: String = format!("keyphrase{}", password);
        let bytes: Vec<u8> = pbkdf2(keyphrase.entropy(), &salt);

        Self { bytes }
    }

    /// Get the seed value as a byte slice
    ///
    /// # Example
    ///
    /// ```
    /// use keyphrase::{KeyPhrase, KeyPhraseType, Language, Seed};
    ///
    /// let word_count: KeyPhraseType = KeyPhraseType::Words15;
    ///
    /// let lang: Language = Language::English;
    ///
    /// let keyphrase: KeyPhrase = KeyPhrase::new(word_count, lang);
    ///
    /// let seed: Seed = Seed::new(&keyphrase, "");
    ///     
    /// let seed_bytes: &[u8] = seed.as_bytes();
    /// ```
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
            write!(f, "{:02x}", byte)?;
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
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_print_upper_hex_correctly() {
        let seed = Seed {
            bytes: vec![1, 10, 16, 255],
        };
        let hex = format!("{:?}", seed);
        assert_eq!(hex, "0x010A10FF")
    }

    #[test]
    fn should_print_lower_hex_correctly() {
        let seed = Seed {
            bytes: vec![255, 16, 10, 1],
        };
        let hex = format!("{:x}", seed);
        assert_eq!(hex, "ff100a01")
    }
}
