# KeyPhrase

KeyPhrase generator for the EARTH Network. KeyPhrases are [BIP0039](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) mnemonics.

```rust
    let keyphrase = KeyPhrase::new(KeyPhraseType::Words12, Language::English);
    let phrase: &str = keyphrase.phrase();
    println!("phrase: {}", phrase);
    // phrase: gather message kiss they verify artwork current else muscle skin connect chicken

    // get the HD wallet seed
    let seed = Seed::new(&keyphrase, "");

    // get the HD wallet seed as raw bytes
    let seed_bytes: &[u8] = seed.as_bytes();

    // print the HD wallet seed as a hex string
    println!("{:X}", seed);
    //AE79FBA1F21ACCB9312E9D77E9A9337F84F1DC33F7E0F806740151858915A56A370A9F225DB3ED21EFC123DBB9E4C8B2F4BC52526FB62AA22F03FC31BD998
```

## Documentation

Full docs are available via

```
cargo doc --no-deps --open
```
