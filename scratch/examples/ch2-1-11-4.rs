fn main() {
    if loop {
        break true;
    } {
        println!("success");
    } else {
        println!("failed");
    }

    if loop {
        break false;
    } {
        println!("success");
    } else {
        println!("failed");
    }
}
