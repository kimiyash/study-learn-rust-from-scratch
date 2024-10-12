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

fn main() {}