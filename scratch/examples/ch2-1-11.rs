use std::ops::DerefMut;

#[derive(Debug, Clone)]
enum List<T> {
    Node { data: T, next: Option<Box<List<T>>> },
}

fn add_node<T: std::clone::Clone>(mut n: &mut List<T>, data: T) {
    loop {
        if let List::Node { ref mut next, .. } = n {
            if let Some(next_) = next {
                n = next_;
            } else {
                *next = Some(Box::new(List::<T>::Node {
                    data: data.clone(),
                    next: None,
                }));
                break;
            }
        }
    }
}

fn main() {
    let mut n1 = List::<u32>::Node {
        data: 0,
        next: None,
    };
    add_node(&mut n1, 10);
    add_node(&mut n1, 20);
    add_node(&mut n1, 30);
    let mut n = &mut n1.clone();
    loop {
        if let List::Node { data, ref mut next } = n {
            println!("{}, {:p}", data, next);
            if let Some(next_) = next {
                n = next_;
            } else {
                break;
            }
        }
    }
}