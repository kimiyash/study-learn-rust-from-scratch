mod b {
    pub struct TypeB;
    mod b_1 {
        pub struct TypeB1 {
            pub n: usize,
            m: usize,
        }

        impl TypeB1 {
            fn g(&self) {}
            pub fn h(&self) {}
        }

        fn f1(p: &super::b_1::TypeB1) {
            println!("{}", p.n);
            println!("{}", p.m);
            p.g();
            p.h();
        }
    }

    pub mod b_2{
        fn f2(p: &super::b_1::TypeB1) {
            println!("{}", p.n);
            // println!("{}", p.m); // m はプライベート
            // p.g(); // g はプライベート
            p.h();
        }
    }
}

mod c {
    mod c_1_outer {
        pub mod c_1_inner {
            pub(crate) struct TypeC1; // 同じクレート内からのみ見える
            pub(super) struct TypeC2; // 親モジュールからのみ見える
            pub(in crate::c::c_1_outer) struct TypeC3; // 親モジュールからのみ見える
            pub(self) struct TypeC4; // プライベートと同義
        }
        fn f() {
            let p1 = c_1_inner::TypeC1;
            let p2 = c_1_inner::TypeC2;
            let p3 = c_1_inner::TypeC3;
            // let p4 = c_1_inner::TypeC4; // プライベートなので見えない
        }
    }

    fn g() {
        let p1 = c_1_outer::c_1_inner::TypeC1;
        // let p2 = c_1_outer::c_1_inner::TypeC2; // エラー
        // let p3 = c_1_outer::c_1_inner::TypeC3; // エラー
        // let p4 = c_1_outer::c_1_inner::TypeC4; // エラー
}
}

fn main() {}