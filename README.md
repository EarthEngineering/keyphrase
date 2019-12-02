# KeyPhrase

KeyPhrase generator for the EARTH Network. KeyPhrases are human readable backup phrase which contains most of the information needed to recreate your EARTH addresses.

Technically KeyPhrases are [BIP0039](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) mnemonics.

```rust
    let keyphrase = KeyPhrase::new(KeyPhraseType::Words12, Language::English);
    let phrase: &str = keyphrase.phrase();
    println!("phrase: {}", phrase);
    // phrase: gather message kiss they verify artwork current else muscle skin connect chicken

    // get the HD wallet seed
    let seed = Seed::new(&keyphrase, "");
    println!("{:X}", seed);
    //AE79FBA1F21ACCB9312E9D77E9A9337F84F1DC33F7E0F806740151858915A56A370A9F225DB3ED21EFC123DBB9E4C8B2F4BC52526FB62AA22F03FC31BD998
```

## Word Lengths

Variable length KeyPhrases are supported from 12 to 24 words and 128 bits to 256 bits respectively. Defaults is 12 words/128 bits.

- 12 (128 bits)
- 15 (160 bits)
- 18 (192 bits)
- 21 (224 bits)
- 24 (256 bits)

## Languages

KeyPhrase supports 8 languages and defaults to English.

- [English](./src/langs/english.txt)
- [Spanish](./src/langs/spanish.txt)
- [Italian](./src/langs/italian.txt)
- [French](./src/langs/french.txt)
- [Korean](./src/langs/korean.txt)
- [Japanese](./src/langs/japanese.txt)
- [Chinese Simplified](./src/langs/chinese_simplified.txt)
- [Chinese Traditional](./src/langs/chinese_traditional.txt)

## Documentation

Full docs are available via

```
cargo doc --no-deps --open
```
