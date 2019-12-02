//!
//! KeyPhrase generator for the [EARTH Network](https://www.earth.engineering). KeyPhrases are [BIP0039](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) mnemonics.
//!
//! ## Quickstart
//!
//! ```rust
//! use keyphrase::{KeyPhrase, KeyPhraseType, Language, Seed};
//!
//! /// create a new randomly generated keyphrase
//! let keyphrase = KeyPhrase::new(KeyPhraseType::Words12, Language::English);
//!
//! /// get the phrase
//! let phrase: &str = keyphrase.phrase();
//! println!("{}", phrase);
//! // grocery unknown bench gold grant slim assist monster laptop cruise hamster any
//!
//! /// get the HD wallet seed
//! let seed = Seed::new(&keyphrase, "");
//!
//! // print the HD wallet seed as a hex string
//! println!("{:X}", seed);
//! // F9BF84A82DD338E08FF79096A8E9ABB3C621B61C64F4906C7FC8BD27B63CEA3773B1EA464CDE3B1272364C6F673713FCB07C97357E75C31EF787E9C251BEDB
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
pub use util::*;
