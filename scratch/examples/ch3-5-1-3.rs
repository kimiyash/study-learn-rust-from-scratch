use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
    thread::sleep,
    time::Duration,
};

fn main() {
    let mut gallery = BTreeMap::new();
    gallery.insert("葛飾北斎", "富岳三十六景");
    gallery.insert("ミシャ", "黄道十二宮");

    let gallery = Arc::new(RwLock::new(gallery));

    let mut hdls = Vec::new();
    for n in 0..200 {
        // 客を表すスレッド
        let gallery = gallery.clone();
        let hdl = std::thread::spawn(move || {
            for l in 0..8 {
                {
                    let gurad = gallery.read().unwrap();
                    // 美術館の展示内容を表示
                    for (key, value) in gurad.iter() {
                        println!("{l}:{n} -> {key}:{value},")
                    }
                }
            }
        });
        hdls.push(hdl);
    }

    let staff = std::thread::spawn(move || {
        for n in 0..4 {
            // 展示内容入れ替え
            if n % 2 == 0 {
                let mut guard = gallery.write().unwrap();
                guard.clear();
                guard.insert("ゴッホ", "星月夜");
                guard.insert("エッシャー", "滝");
                sleep(Duration::from_secs(1));
            } else {
                let mut guard = gallery.write().unwrap();
                guard.clear();
                guard.insert("葛飾北斎", "富岳三十六景");
                guard.insert("ミシャ", "黄道十二宮");
                sleep(Duration::from_secs(1));
            }
            // sleep(Duration::from_secs(2));
        }
    });

    for hdl in hdls {
        hdl.join().unwrap();
    }
    staff.join().unwrap();

}
