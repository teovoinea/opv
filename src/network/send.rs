use std;
use std::net::UdpSocket;
use std::sync::mpsc::{
    Receiver
};
use bincode::{serialize, Infinite};

use transcode::Chunk;

pub fn net_send(src: &'static str, dst: &'static str,receiver: Receiver<Vec<Chunk>>) {
    let socket = UdpSocket::bind(src).expect("couldn't bind to address");
    let netthread = std::thread::Builder::new().name("Network Send".to_string()).spawn(move || {
        loop {
            if let Ok(chunks) = receiver.try_recv() {
                for chunk in chunks {
                    let res = serialize(&chunk, Infinite);
                    match res {
                        Ok(bytes) => {
                            socket.send_to(bytes.as_slice(), dst).expect("couldn't send data");
                        }

                        Err(_) => {
                            //TODO(teo): error handling
                        }
                    }
                }
            }
        }
    });
    //TODO(teo): thread clean up
    //netthread.join().unwrap();
}