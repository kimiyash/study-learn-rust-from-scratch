struct Buffer<const S: usize> {
    buf: [u8; S],
}

fn main() {
    // let buf = Buffer::<128> { buf: [0; 127] };
    //     error[E0308]: mismatched types
    //     --> examples/ch2-1-11-2.rs:6:36
    //      |
    //    6 |     let buf = Buffer::<128> { buf: [0; 127] };
    //      |                                    ^^^^^^^^ expected an array with a fixed size of 128 elements, found one with 127 elements

    let buf = Buffer::<128> { buf: [0; 128] };
}
