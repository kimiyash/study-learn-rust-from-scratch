use std::sync::Arc;

fn main() {
    let v = Arc::new(vec![1, 2, 3]);
    let x = v.clone();
    let y = v.clone();
}
