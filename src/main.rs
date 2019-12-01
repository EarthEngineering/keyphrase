use keyphrase::{Language, Mnemonic, MnemonicType};

fn main() {
    better_panic::install();
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::Japanese);
    let phrase: &str = mnemonic.phrase();
    println!("phrase: {}", phrase);
}
