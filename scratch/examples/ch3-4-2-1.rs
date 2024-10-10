fn main() {
    let mut a = 10;
    let b = &mut a;
    // if a == 10 { // ここでエラー
    //     println!("");
    // }
    // let c = &a; // ここでもエラー
    //上記は &mu b を利用しているためすべてエラーとなる
    *b = 1;
    if a == 1 {
        println!("{a}");
    }
}