use rand::Rng;

fn main() {
    let alphabet_upper: Vec<char> = fill_alphabet_upper();
    let alphabet_lower: Vec<char> = fill_alphabet_lower();
    let numbers: Vec<u8> = fill_numbers();

    for _i in 0..14 {
        let rng1: u8 = rand::thread_rng().gen_range(1..=3);

        match rng1 {
            3 => {
                    let randchar = rand::thread_rng().gen_range(1..=25);
                    print!("{}", alphabet_lower[randchar]);
                }
            2 => {
                    let randchar = rand::thread_rng().gen_range(1..=9);
                    print!("{}", numbers[randchar]);
                },
            1 => {
                    let randchar = rand::thread_rng().gen_range(1..=25);
                    print!("{}", alphabet_upper[randchar]);
                }
            _ => println!("how did this happen?")
        }
    }

    println!("");
}

fn fill_alphabet_upper() -> Vec<char> {
    let alphabet = (b'A'..=b'Z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>();

    return alphabet
}

fn fill_alphabet_lower() -> Vec<char> {
    let alphabet = (b'a'..=b'z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>();

    return alphabet
}

fn fill_numbers() -> Vec<u8> {
    let mut numbers: Vec<u8> = Vec::new();

    for i in 0..10 {
        numbers.push(i);
    }

    return numbers;
}
