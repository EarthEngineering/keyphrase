use keyphrase::{KeyPhrase, KeyPhraseType, Language, Seed};

fn main() {
    better_panic::install();
    let keyphrase = KeyPhrase::new(KeyPhraseType::Words12, Language::English);
    let phrase: &str = keyphrase.phrase();
    println!("phrase: {}", phrase);

    // get the HD wallet seed
    let seed = Seed::new(&keyphrase, "");

    // get the HD wallet seed as raw bytes
    let seed_bytes: &[u8] = seed.as_bytes();

    // print the HD wallet seed as a hex string
    println!("{:X}", seed);
}
