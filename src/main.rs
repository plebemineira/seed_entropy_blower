fn counter(c: u8) {
    for i in 0..c {
        println!("{}...", i+1);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn main() {
    println!("\n🌬️🎙️ seed_entropy_blower 🎲🗝️\n\nbe prepared to generate some entropy by blowing into your mic in 5s.");
    counter(5);
    println!("NOW! Keep blowing for 10s 🌬️🎙️");
    std::thread::spawn(|| {
        counter(10);
    });

    let entropy_blower = seed_entropy_blower::entropy::SeedEntropyBlower::new();
    let bip39 = entropy_blower.get_bip39_seed_phrase();

    println!("\nBIP39 seed phrase ✍️🗝️");
    println!("position|word|number");
    let mut i = 1;
    for word in bip39.word_iter() {
        println!("{}|{}|{}", i, word, bip39::Language::English.find_word(word).expect("this should be a valid word")+1);
        i+=1;
    }
}