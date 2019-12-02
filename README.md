# KeyPhrase

KeyPhrase generator for the EARTH Network. KeyPhrases are human readable backup phrase which contains most of the information needed to recreate your EARTH addresses.

Technically KeyPhrases are [BIP0039](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) mnemonics.

```rust
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

    // English:
    // KeyPhrase: never snack lazy weasel fault online obvious seminar coin come hazard seat
    // Root Seed: 3C9C95B810C2D1EA785D9755B8461A8AA254FB23CFFB3C665C38DAA7F570725E547E554FFEB2D2E38D966FF4A6008B75F733FEF9899134889571B7F99B358
    // ----------
    // Korean:
    //KeyPhrase: 안부 모니터 장애인 대출 지원 본격적 서적 다행 관점 부족 유치원 갈비
    // ----------
    // Italian:
    // KeyPhrase: meno elevare diploma tralcio montato servire gittata certo garbo ombelico sfumare sguardo
    // ----------
    // Chinese Traditional:
    // KeyPhrase: 廟 牙 錢 療 健 董 疆 胸 冊 弱 幾 凍 改 喬 叔 冷 山 慘 溝 呵 長 趨 鋪 跳
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
