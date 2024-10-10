use std::ops::DerefMut;

#[derive(Debug, Clone)]
enum List<T> {
    Node { data: T, next: Option<Box<List<T>>> },
}

// fn add_node<T>(n: List<T>, data: T) {
//     let n_ = List::<T>::Node {
//         data: data.clone()
//         next: 
//     }
// }

fn main() {
    let n1 = List::<u32>::Node {
        data: 0,
        next: None,
    };
    let n2 = List::<u32>::Node {
        data: 10,
        next: Some(Box::<List<u32>>::new(n1)),
    };
    let n3 = List::<u32>::Node {
        data: 20,
        next: Some(Box::<List<u32>>::new(n2)),
    };
    let mut n = &mut n3.clone();
    loop {
        if let List::Node { data, ref mut next } = n {
            println!("{}, {:p}", data, next);
            let next_ref = next.as_deref_mut();
            if let Some(next_) = next_ref {
                n = next_;
            } else {
                break;
            }
        }
    }

    if let List::Node { data, next } = n3 {
        println!("{} {:p}", data, &next);
    }
}