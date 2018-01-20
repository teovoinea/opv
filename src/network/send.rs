use std;
use std::net::UdpSocket;
use std::sync::mpsc::{
    Receiver
};
use bincode::{serialize, Infinite};

use transcode::Chunk;

pub fn net_send(src: &str, dst: &str, receiver: Receiver<Vec<Chunk>>) {
    let socket = UdpSocket::bind(src).expect("couldn't bind to address");
    let dst_clone = String::from(dst);
    let netthread = std::thread::Builder::new().name("Network Send".to_string()).spawn(move || {
        loop {
            if let Ok(chunks) = receiver.try_recv() {
                for chunk in chunks {
                    let res = serialize(&chunk, Infinite);
                    match res {
                        Ok(bytes) => {
                            if let Ok(sent) = socket.send_to(bytes.as_slice(), &dst_clone) {
                                //gucci
                            }
                            else {
                                //println!("Error sending data");
                            }
                        }

                        Err(_) => {
                            //TODO(teo): error handling
                        }
                    }
                }
            }
            else {
                //println!("Error receving data for network");
            }
        }
    });
    //TODO(teo): thread clean up
    //netthread.join().unwrap();
}