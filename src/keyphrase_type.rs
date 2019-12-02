use crate::error::ErrorKind;
use failure::Error;
use std::fmt;

const ENTROPY_OFFSET: usize = 8;

/// Determines the number of words that will be present in a [`KeyPhrase`][KeyPhrase] phrase
///
/// Also directly affects the amount of entropy that will be used to create a [`KeyPhrase`][KeyPhrase],
/// and therefore the cryptographic strength of the HD wallet keys/addresses that can be derived from
/// it using the [`Seed`][Seed].
///
/// For example, a 12 word keyphrase is essentially a friendly representation of a 128-bit key,
/// while a 24 word keyphrase is essentially a 256-bit key.
///
/// If you know you want a specific phrase length, you can use the enum variant directly, for example
/// `KeyPhraseType::Words12`.
///
/// You can also get a `KeyPhraseType` that corresponds to one of the standard BIP39 key sizes by
/// passing arbitrary `usize` values:
///
/// ```
/// use keyphrase::{KeyPhraseType};
///
/// let keyphrase_type = KeyPhraseType::for_key_size(128).unwrap();
/// ```
///
/// [KeyPhraseType]: ../keyphrase_type/struct.KeyPhraseType.html
/// [KeyPhrase]: ../keyphrase/struct.KeyPhrase.html
/// [Seed]: ../seed/struct.Seed.html
///
#[derive(Debug, Copy, Clone)]
pub enum KeyPhraseType {
    //  ... = (entropy_bits << ...)   | checksum_bits
    Words12 = (128 << ENTROPY_OFFSET) | 4,
    Words15 = (160 << ENTROPY_OFFSET) | 5,
    Words18 = (192 << ENTROPY_OFFSET) | 6,
    Words21 = (224 << ENTROPY_OFFSET) | 7,
    Words24 = (256 << ENTROPY_OFFSET) | 8,
}

impl KeyPhraseType {
    /// Get a `KeyPhraseType` for a keyphrase with a specific number of words
    ///
    /// Specifying a word count not provided for by the BIP39 standard will return an `Error`
    /// of kind `ErrorKind::InvalidWordLength`.
    ///
    /// # Example
    /// ```
    /// use keyphrase::{KeyPhraseType};
    ///
    /// let keyphrase_type = KeyPhraseType::for_word_count(12).unwrap();
    /// ```
    pub fn for_word_count(size: usize) -> Result<KeyPhraseType, Error> {
        let keyphrase_type = match size {
            12 => KeyPhraseType::Words12,
            15 => KeyPhraseType::Words15,
            18 => KeyPhraseType::Words18,
            21 => KeyPhraseType::Words21,
            24 => KeyPhraseType::Words24,
            _ => Err(ErrorKind::InvalidWordLength(size))?,
        };

        Ok(keyphrase_type)
    }

    /// Get a `KeyPhraseType` for a keyphrase representing the given key size as bits
    ///
    /// Specifying a key size not provided for by the BIP39 standard will return an `Error`
    /// of kind `ErrorKind::InvalidKeysize`.
    ///
    /// # Example
    /// ```
    /// use keyphrase::{KeyPhraseType};
    ///
    /// let keyphrase_type = KeyPhraseType::for_key_size(128).unwrap();
    /// ```
    pub fn for_key_size(size: usize) -> Result<KeyPhraseType, Error> {
        let keyphrase_type = match size {
            128 => KeyPhraseType::Words12,
            160 => KeyPhraseType::Words15,
            192 => KeyPhraseType::Words18,
            224 => KeyPhraseType::Words21,
            256 => KeyPhraseType::Words24,
            _ => Err(ErrorKind::InvalidKeysize(size))?,
        };

        Ok(keyphrase_type)
    }

    /// Get a `KeyPhraseType` for an existing keyphrase
    ///
    /// This can be used when you need information about a keyphrase based on the number of
    /// words, for example you can get the entropy value using [`KeyPhraseType::entropy_bits`][KeyPhraseType::entropy_bits()].
    ///
    /// Specifying a phrase that does not match one of the standard BIP39 phrase lengths will return
    /// an `Error` of kind `ErrorKind::InvalidWordLength`. The phrase will not be validated in any
    /// other way.
    ///
    /// # Example
    /// ```
    /// use keyphrase::{KeyPhraseType};
    ///
    /// let test_keyphrase = "park remain person kitchen mule spell knee armed position rail grid ankle";
    ///
    /// let keyphrase_type = KeyPhraseType::for_phrase(test_keyphrase).unwrap();
    ///
    /// let entropy_bits = keyphrase_type.entropy_bits();
    /// ```
    ///
    /// [KeyPhraseType::entropy_bits()]: ./enum.KeyPhraseType.html#method.entropy_bits
    pub fn for_phrase(phrase: &str) -> Result<KeyPhraseType, Error> {
        let word_count = phrase.split(" ").count();

        Self::for_word_count(word_count)
    }

    /// Return the number of entropy+checksum bits
    ///
    ///
    /// # Example
    /// ```
    /// use keyphrase::{KeyPhraseType};
    ///
    /// let test_keyphrase = "park remain person kitchen mule spell knee armed position rail grid ankle";
    ///
    /// let keyphrase_type = KeyPhraseType::for_phrase(test_keyphrase).unwrap();
    ///
    /// let total_bits = keyphrase_type.total_bits();
    /// ```
    pub fn total_bits(&self) -> usize {
        self.entropy_bits() + self.checksum_bits() as usize
    }

    /// Return the number of entropy bits
    ///
    ///
    /// # Example
    /// ```
    /// use keyphrase::{KeyPhraseType};
    ///
    /// let test_keyphrase = "park remain person kitchen mule spell knee armed position rail grid ankle";
    ///
    /// let keyphrase_type = KeyPhraseType::for_phrase(test_keyphrase).unwrap();
    ///
    /// let entropy_bits = keyphrase_type.entropy_bits();
    /// ```
    pub fn entropy_bits(&self) -> usize {
        (*self as usize) >> ENTROPY_OFFSET
    }

    /// Return the number of checksum bits
    ///
    ///
    /// # Example
    /// ```
    /// use keyphrase::{KeyPhraseType};
    ///
    /// let test_keyphrase = "park remain person kitchen mule spell knee armed position rail grid ankle";
    ///
    /// let keyphrase_type = KeyPhraseType::for_phrase(test_keyphrase).unwrap();
    ///
    /// let checksum_bits = keyphrase_type.checksum_bits();
    /// ```
    pub fn checksum_bits(&self) -> u8 {
        (*self as usize) as u8
    }

    /// Return the number of words
    ///
    ///
    /// # Example
    /// ```
    /// use keyphrase::{KeyPhraseType};
    ///
    /// let keyphrase_type = KeyPhraseType::Words12;
    ///
    /// let word_count = keyphrase_type.word_count();
    /// ```
    pub fn word_count(&self) -> usize {
        self.total_bits() / 11
    }
}

impl Default for KeyPhraseType {
    fn default() -> KeyPhraseType {
        KeyPhraseType::Words12
    }
}

impl fmt::Display for KeyPhraseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} words ({}bits)",
            self.word_count(),
            self.entropy_bits()
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn word_count() {
        assert_eq!(KeyPhraseType::Words12.word_count(), 12);
        assert_eq!(KeyPhraseType::Words15.word_count(), 15);
        assert_eq!(KeyPhraseType::Words18.word_count(), 18);
        assert_eq!(KeyPhraseType::Words21.word_count(), 21);
        assert_eq!(KeyPhraseType::Words24.word_count(), 24);
    }

    #[test]
    fn entropy_bits() {
        assert_eq!(KeyPhraseType::Words12.entropy_bits(), 128);
        assert_eq!(KeyPhraseType::Words15.entropy_bits(), 160);
        assert_eq!(KeyPhraseType::Words18.entropy_bits(), 192);
        assert_eq!(KeyPhraseType::Words21.entropy_bits(), 224);
        assert_eq!(KeyPhraseType::Words24.entropy_bits(), 256);
    }

    #[test]
    fn checksum_bits() {
        assert_eq!(KeyPhraseType::Words12.checksum_bits(), 4);
        assert_eq!(KeyPhraseType::Words15.checksum_bits(), 5);
        assert_eq!(KeyPhraseType::Words18.checksum_bits(), 6);
        assert_eq!(KeyPhraseType::Words21.checksum_bits(), 7);
        assert_eq!(KeyPhraseType::Words24.checksum_bits(), 8);
    }
}
