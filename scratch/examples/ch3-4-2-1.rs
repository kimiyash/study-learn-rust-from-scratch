#[derive(Debug)]
struct XY {
    x: Vec<i32>,
    y: Vec<i32>,
}

fn main() {
    // 構造体の例
    let mut xy = XY {
        x: vec![1, 2, 3],
        y: Vec::new(),
    };
    // xy のフィールド x を借用すると、xy全体も借用されるのでコンパイルエラー、とならない…
    for elm in xy.x.iter() {
        xy.y.push(*elm * *elm);
    }
    println!("{:?}", xy);
    // コンパイルエラーとなる場合下記のように書くといいらしい
    let XY { x, y } = &mut xy;
    for elm in x {
        y.push(*elm * *elm);
    }
    println!("{:?}", xy);

    // 変数借用規則の例
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
    if a == 1 {
        // ここなら a は読み込める
        println!("{a}");
    }
}
