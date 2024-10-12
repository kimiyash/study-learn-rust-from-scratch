use std::iter::Iterator;

// 不変イテレータ
struct ListIter<'a, T> {
    elm: &'a List<T>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.elm {
            List::Node { data, next } => {
                self.elm = next;
                Some(data)
            },
            List::Nil => None,
        }
    }
}

#[derive(Debug)]
enum List<T> {
    Node { data: T, next: Box<List<T>> },
    Nil,
}

impl<T> List<T> {
    fn new() -> List<T> {
        List::Nil
    }

    // 自身を消費して最初のノードに挿入しそれをかえすメソッド
    fn cons(self, data: T) -> List<T> {
        List::Node {
            data,
            next: Box::new(self),
        }
    }

    // 不変イテレータを返す
    fn iter(&self) -> ListIter<T> {
        ListIter { elm: self }
    }
}

fn main() {
    let list = List::new();
    let list = list.cons(1);
    let list = list.cons(2);
    let list = list.cons(3);

    for data in list.iter() {
        println!("{data}");
    }

    let mut it = list.iter();
    println!("{}", it.next().unwrap());
    println!("{}", it.next().unwrap());
    println!("{}", it.next().unwrap());
    println!("{}", it.next().unwrap_or(&999));
}
