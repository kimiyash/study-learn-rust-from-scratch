fn main() {
    let mut a = 10;
    let b = &mut a;
    // if a == 10 { // ここでエラー
    //     println!("");
    // }
    // let c = &a; // ここでもエラー
    //上記は &mu b を利用しているためすべてエラーとなる
    let d = b;
    //*b = 1; // エラー、&mut はムーブセマンティクス
    *d = 1;
    // if a == 1 {  // 下の行で d の中を参照しているので a は読み込めない
    //     // println!("{a}"); 
    // }
    if *d == 1 {
        println!("{d}");
    }
    if a == 1 {  // ここなら a は読み込める
        println!("{a}"); 
    }
}