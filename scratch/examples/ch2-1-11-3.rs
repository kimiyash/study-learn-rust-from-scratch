fn hello() {
    #[derive(Debug)]
    struct Msg {
        msg1: &'static str,
        msg2: &'static str,
    }

    fn print_msg(msg: &Msg) {
        println!("{:?}", msg);
    }

    let msg = Msg {msg1: "hello", msg2: "world"};
    print_msg(&msg);
}

fn main() {
    hello();
}