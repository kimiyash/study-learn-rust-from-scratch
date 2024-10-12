use std::{io::SeekFrom, iter::Iterator};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::prelude::*, path::Path};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    let list = List::new().cons(1).cons(2).cons(3);

    for data in list.iter() {
        println!("{data}");
    }

    let mut it = list.iter();
    println!("{}", it.next().unwrap());
    println!("{}", it.next().unwrap());
    println!("{}", it.next().unwrap());
    println!("{}", it.next().unwrap_or(&999));

    // Jsonにシリアライズ
    let js = serde_json::to_string(&list).unwrap();
    println!("{js}");

    // YAMLにシリアライズ
    let yaml = serde_yaml::to_string(&list).unwrap();
    println!("{yaml}");

    // MessagePackにシリアライズ
    let msgpack = rmp_serde::to_vec(&list).unwrap();
    println!("MessagePack: {} bytes", msgpack.len());

    // JSONからデシリアライズ
    let js = serde_json::from_str::<List<i32>>(&js).unwrap();
    println!("{:#?}", js);

    // YAML からデシリアライズ
    let yaml = serde_yaml::from_str::<List<i32>>(&yaml).unwrap();
    println!("{:#?}", yaml);

    // MessagePack からデシリアライズ
    let msgpack = rmp_serde::from_slice::<List<i32>>(&msgpack).unwrap();
    println!("{:#?}", msgpack);

    let yaml = serde_yaml::to_string(&list).unwrap();
    let path = Path::new("test.yml");
    let mut f = File::create(path).unwrap();
    f.write_all(yaml.as_bytes()).unwrap();

    println!();
    let mut f = File::open("test.yml").unwrap();
    let mut yaml = String::new();
    f.read_to_string(&mut yaml).unwrap();
    println!("{yaml}");

    println!();
    let mut f = File::open("test.yml").unwrap();
    let mut v = Vec::new();
    f.read_to_end(&mut v).unwrap();
    let yaml = String::from_utf8(v).unwrap();
    println!("{yaml}");
}
