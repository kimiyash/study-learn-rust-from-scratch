#[derive(Debug, Clone)]
enum List<T> {
    Node { data: T, next: Option<Box<List<T>>> },
}

fn add_node<T: std::clone::Clone>(node: &mut List<T>, data: T) -> &mut List<T> {
    let mut current = node;
    loop {
        let List::Node { ref mut next, .. } = current;
        if let Some(next) = next {
            current = next;
        } else {
            *next = Some(Box::new(List::<T>::Node {
                data: data.clone(),
                next: None,
            }));
            break next.as_mut().unwrap()
        }
    }
}

fn main() {
    let mut n1 = List::<u32>::Node {
        data: 0,
        next: None,
    };
    let new_node = add_node(&mut n1, 10);
    let new_node = add_node(new_node, 20);
    let new_node = add_node(new_node, 30);
    add_node(new_node, 40);
    add_node(new_node, 50);

    let mut n = &mut n1.clone();
    loop {
        let List::Node { data, next } = n;

        println!("{}, {:p}", data, next);

        if let Some(next_) = next {
            n = next_;
        } else {
            break;
        }
    }
}