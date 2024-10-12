fn worker() -> u32 {
    println!("worker!");
    100
}

fn main() {
    let handler = std::thread::spawn(worker);
    match handler.join() {
        Ok(n) => println!("{n}"),
        Err(e) => println!("{:?}", e),
    }

    // sync_channel はバッファがつまったら送信側がブロックする
    // 一方 channel はバッファのサイズに制限がないためブロックされない
    let (tx, rx) = std::sync::mpsc::sync_channel(64);
    let handler = std::thread::spawn(move || match rx.recv() {
        Ok((x, y)) => println!("{} {}", x, y),
        Err(e) => eprintln!("{e}"),
    });
    if let Err(e) = tx.send((10, 20)) {
        eprintln!("{e}");
    }
    if let Err(e) = handler.join() {
        eprintln!("{:?}", e);
    }
}
