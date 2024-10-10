fn main() {
    println!("{}", a() || b());
    println!("{}", a() | b());
    println!("{}", c().is_ascii() || a());
    println!("{}", c().is_ascii() | a());
}

fn a() -> bool {
    println!("a");
    true
}

fn b() -> bool {
    println!("b");
    true
}

fn c() -> char {
    println!("c");
    'c'
}