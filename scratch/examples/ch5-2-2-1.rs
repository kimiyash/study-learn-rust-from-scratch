mod a {
    struct TypeA {
        // a1: a_1::TypeA1, 子のプライベートな要素はみえない
        a2: Box<a_2::TypeA2>, // 子のパブリックな要素は見える
    }

    mod a_1 {
        struct TypeA1 {
            // 親がみえるものは見える
            a: Box<super::TypeA>,
            a2: Box<super::a_2::TypeA2>,
        }
    }
    mod a_2 {
        pub struct TypeA2 {
            // 親がみえるものは見える
            a: Box<super::TypeA>,
            // a1: super::a_1::TypeA1, // 親がみえないものは見えない
        }
    }
}

mod b {
    pub struct TypeB;
    mod b_1 {
        pub struct TypeB1;
    }
    pub mod b_2 {
        pub struct TypeB2;
    }
}


fn main() {
    // let a = a::TypeA; // 子のプライベートな要素は見えない
    let b = b::TypeB;

    // let b1 = b::b_1::TypeB1; // このプライベートなモジュール b_1 は見えない
    let b2 = b::b_2::TypeB2; // パブリックな孫 b_2 のパブリックな要素 TypeB2 は見える
}