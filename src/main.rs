use keyphrase::{KeyPhrase, KeyPhraseType, Language, Seed};

fn main() {
    better_panic::install();

    println!("English:");
    let word_count: KeyPhraseType = KeyPhraseType::Words15;
    let lang: Language = Language::English;
    let keyphrase: KeyPhrase = KeyPhrase::new(word_count, lang);
    let phrase: &str = keyphrase.phrase();
    println!("KeyPhrase: {:#?}", phrase);
    let lang: Language = keyphrase.language();
    println!("lang: {:#?}", lang);

    // let seed: Seed = Seed::new(&keyphrase, "");

    // get the HD wallet seed as raw bytes
    // let seed_bytes: &[u8] = seed.as_bytes();
    // println!("seed_bytes: {:#?}", seed_bytes);

    // print the HD wallet seed as a hex string
    // println!("{:X}", seed_bytes);
}
