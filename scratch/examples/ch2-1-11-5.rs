// use std::assert_matches::assert_matches;

#[derive(Debug)]
enum Storage {
    HDD { size: u32, rpm: u32 },
    SSD(u32),
}

#[derive(Debug)]
struct PCSpec {
    cpus: u16,
    memory: u32,
    storage: Storage,
}

fn main() {
    let spec = PCSpec {
        cpus: 8,
        memory: 3,
        storage: Storage::SSD(512),
    };
    match &spec { // match で値が消費されてしまうので参照をつかう
        PCSpec {
            storage: Storage::SSD(512),
            ..
        } => {
            println!("512GiB SSD");
        }
        PCSpec {
            cpus: 4 | 8, // 4 か 8
            memory: m, // mにメモリサイズが記録される
            storage: _,
        } => {
            println!("4 or 8 CPUs");
            println!("{}GiB memory", *m);
        }
        PCSpec { memory: m, ..} if *m < 4 => {
            // m が 4未満
            println!("4Gib より少ないメモリ")
        }
        _ => ()
    }

    // let a = Some(1);
    // assert_matches!(a, Some(_));
    //     error[E0658]: use of unstable library feature 'assert_matches'
    //     --> examples/ch2-1-11-5.rs:44:5
    //      |
    //   44 |     assert_matches!(a, Some(_));
    //      |     ^^^^^^^^^^^^^^
    //      |
    //      = note: see issue #82775 <https://github.com/rust-lang/rust/issues/82775> for more information
  
  
}