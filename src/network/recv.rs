use std;
use std::net::UdpSocket;
use bincode::{deserialize};
use std::sync::mpsc::{
    Sender
};
use image::{
    ImageBuffer,
    Rgba
};
use transcode::{
    Chunk,
    decode
};

pub fn net_recv(address: &str, sender: Sender<ImageBuffer<Rgba<u8>, Vec<u8>>>) {
    {
        let socket = UdpSocket::bind(address).expect("couldn't bind to address");
        let netthread = std::thread::Builder::new().name("Network Receive".to_string()).spawn(move || {
            //let mut frame_buf = vec![0; 1228800];
            let mut chunk_buf: Vec<Chunk> = Vec::new();
            loop {
                let mut buf = [0; 1516];
                match socket.recv(&mut buf) {
                    Ok(received) => {
                        let decoded = deserialize(&buf);
                        match decoded {
                            Ok(chunk) => {
                                chunk_buf.push(chunk);
                                if chunk_buf.len() == 820 {
                                    let frame = decode::chunks_to_frame(chunk_buf);
                                    chunk_buf = Vec::new();
                                    sender.send(frame);
                                }
                            }

                            Err(_) => {
                                println!("Error decoding chunk");
                                //TODO(teo): error handling
                            }
                        }

                        //received this many bytes
                        //println!("{:?}", received);
                        //frame_buf.extend(buf.iter());
                        /*
                        println!("chunk #{:?}", buf[0]);
                        println!("packet size{:?}", buf.len());
                        let buffer_index: usize = buf[0] as usize * 1500;
                        for i in buffer_index..buffer_index+buf.len() - 1 {
                            frame_buf[i] = buf[i - buffer_index];
                        }
                        let frame: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(640, 480, frame_buf.clone()).unwrap();
                        remotesender.send(frame);
                        println!("chunk #{:?}", buf[0]);
                        frame_buf.extend(buf.iter());
                        if frame_buf.len() >= 1228800 as usize {
                            println!("we got a frame fam");
                            let mut new_frame = frame_buf.split_off(1228800);
                            let frame: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(640, 480, frame_buf).unwrap();
                            sender.send(frame);
                            frame_buf = new_frame;
                        }*/
                    }
                    Err(e) => {
                        println!("error reading from remote socket {:?}", e);
                    }
                }
            }
        });
    }
    //netthread.join().unwrap();
}