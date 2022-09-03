use rand::Rng;

fn main() {
    let alphabet_upper: Vec<char> = fill_alphabet_upper();
    let alphabet_lower: Vec<char> = fill_alphabet_lower();
    let numbers: Vec<u8> = fill_numbers();
    let symbol: Vec<char> = vec!['@', '!', '#', '$', '%', '&', '*', '-', '_'];

    for _i in 0..14 {
        let rng1: u8 = rand::thread_rng().gen_range(1..=4);

        match rng1 {
            4 => print_char(&symbol),
            3 => print_char(&alphabet_lower),
            2 => print_char(&numbers),
            1 => print_char(&alphabet_upper),
            _ => println!("how did this happen?"),
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

fn print_char<T: std::fmt::Display>(x: &Vec<T>) {
    let randchar = rand::thread_rng().gen_range(1..=x.len()-1);
    print!("\x1b[93m{}\x1b[0m", x[randchar]);
}
