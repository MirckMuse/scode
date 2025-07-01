mod char;

fn main() {
    let char = "中";
    println!("{}", char.chars().nth(0).unwrap() as u32);
}