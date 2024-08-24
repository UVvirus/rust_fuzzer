use rand::Rng;

pub fn generate_random_number(min: u32, max: u32)-> u32{
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(min..=max);
    random_number
}

pub fn generate_random_word(word_length: u32) -> String{
    let letters='a'..='z';
    let mut rng = rand::thread_rng();
    let random_word = (0..word_length).map(|_| {
        let char = rng.gen_range(letters.clone());
        char
    }).collect();
    println!("Random word: {}", random_word);
    random_word
}
