use keyphrase::{KeyPhrase, KeyPhraseType, Language, Seed};

fn main() {
    better_panic::install();

    let word_count: KeyPhraseType = KeyPhraseType::Words12;
    //    let lang: Language = Language::English;
    let lang: Language = Language::ChineseSimplified;
    let keyphrase: KeyPhrase = KeyPhrase::new(word_count, lang);
    let phrase: &str = keyphrase.phrase();
    println!("{:#?}", phrase);

    let seed: Seed = Seed::new(&keyphrase, "");
    println!("{:X}", seed);

    // get the HD wallet seed as raw bytes
    // let seed_bytes: u[u8] = seed.as_bytes();

    // print the HD wallet seed as a hex string
    // println!("{:#?}", seed_bytes);
    // println!("{:#?}", seed_bytes.len());
}
