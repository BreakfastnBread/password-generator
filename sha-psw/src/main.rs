use sha256::digest;
use rand::Rng;
fn main() {
    let letters = [ "a", "b", "c", "d", "e", "f", "g", "h", "i"];
    let mut rand: Vec<&str> = Vec::new();
    for _i in 0..10 {
        let r = rand::thread_rng().gen_range(1..=4);
        rand.push(&letters[r]);}
    let sentence = format!("{}{}{}{}{}{}{}{}{}{}",&rand[0],&rand[1],&rand[2],&rand[3],&rand[4],&rand[5],&rand[6],&rand[7],&rand[8],&rand[09]);
    let val = digest(&sentence);
    print!("{}", val);
}
