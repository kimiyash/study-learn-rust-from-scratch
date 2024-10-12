use std::sync::Arc;

fn main() {
    let arc = Arc::new(vec![1, 2, 3]);
    let _a1 = arc.clone();
    let _a2 = arc.clone();
    let _a3 = arc.clone();
}

