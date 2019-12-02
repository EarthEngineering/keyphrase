use keyphrase::{KeyPhrase, KeyPhraseType, Language, Seed};

fn main() {
    better_panic::install();

    println!("English:");
    let keyphrase = KeyPhrase::new(KeyPhraseType::Words12, Language::English);
    let phrase: &str = keyphrase.phrase();
    println!("KeyPhrase: {}", phrase);

    // get the HD wallet seed
    let seed = Seed::new(&keyphrase, "");
    println!("Root Seed: {:X}", seed);
    println!("----------");

    println!("Korean:");
    let keyphrase = KeyPhrase::new(KeyPhraseType::Words12, Language::Korean);
    let phrase: &str = keyphrase.phrase();
    println!("KeyPhrase: {}", phrase);
    println!("----------");

    println!("Italian:");
    let keyphrase = KeyPhrase::new(KeyPhraseType::Words12, Language::Italian);
    let phrase: &str = keyphrase.phrase();
    println!("KeyPhrase: {}", phrase);
    println!("----------");

    println!("Chinese Traditional:");
    let keyphrase = KeyPhrase::new(KeyPhraseType::Words24, Language::ChineseTraditional);
    let phrase: &str = keyphrase.phrase();
    println!("KeyPhrase: {}", phrase);
    println!("----------");

    // get the HD wallet seed as raw bytes
    // let seed_bytes: &[u8] = seed.as_bytes();

    // print the HD wallet seed as a hex string
    // println!("{:X}", seed_bytes);
}
