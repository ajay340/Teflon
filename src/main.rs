pub mod vm;
pub mod instructions;

fn main() {
    let x = 6;
    let y = 7;

    match x == y {
        true => println!("true"),
        false => println!("false"),
    }

    println!("{}", 15 % 2);
}
