mod dmx_send;
use std::{thread, sync, time};


fn main() {
    // data can be arbitrary number of u8 values, so use Vec<u8>
    // we do NOT use a reference, because ownership needs to pass to new thread
    let (tx, rx) = sync::mpsc::channel::<Vec<u8>>();

    dmx_send::spawn_receiver(rx);

    for i in 0..100 {
        println!("hi number {} from the main thread", i);
        let x: u8;
        let v: Vec<u8>;
        match i {
            0..=24 => { x = i * 10 ; v = vec![x, 0x00, 0x00, 0x00] },
            25..=49 => { x = (i - 25) * 10; v = vec![0x00, x, 0x00, 0x00] },
            50..=74 => { x = (i - 50) * 10; v = vec![0x00, 0x00, x, 0x00] },
            75..=98 => { x = (i - 75) * 10; v = vec![0x00, 0x00, 0x00, x] },
            99..=u8::MAX => { v = vec![0x00, 0x00, 0x00, 0x00] },
        }
        tx.send(v).unwrap();
        thread::sleep(time::Duration::from_millis(100));
    }

    println!("finished (with dmx_send module");
}