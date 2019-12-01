//!
//! Coder/Decoder for Addresses on the [EARTH Network](https://www.earth.engineering).
//!
//! ## Quickstart
//!
//! ```rust
//! use keyphrase::{KeyPhrase, KeyPhraseType, Language, Seed};
//!
//! /// create a new randomly generated mnemonic phrase
//! let mnemonic = KeyPhrase::new(KeyPhraseType::Words12, Language::English);
//!
//! /// get the phrase
//! let phrase: &str = mnemonic.phrase();
//! println!("phrase: {}", phrase);
//!
//! /// get the HD wallet seed
//! let seed = Seed::new(&mnemonic, "");
//!
//! // get the HD wallet seed as raw bytes
//! let seed_bytes: &[u8] = seed.as_bytes();
//!
//! // print the HD wallet seed as a hex string
//! println!("{:X}", seed);
//! ```
//!
#[macro_use]
extern crate failure;
#[macro_use]
extern crate once_cell;
extern crate hashbrown;
extern crate hmac;
extern crate pbkdf2;
extern crate sha2;

mod error;
mod keyphrase;
mod keyphrase_type;
mod language;
mod seed;
mod util;

mod crypto;

pub use self::keyphrase::KeyPhrase;
pub use error::ErrorKind;
pub use keyphrase_type::KeyPhraseType;
pub use language::Language;
pub use seed::Seed;
