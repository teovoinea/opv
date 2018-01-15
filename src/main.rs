extern crate camera_capture;
extern crate piston_window;
extern crate image;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
#[macro_use]
extern crate lazy_static;
extern crate stun;
//extern crate portaudio;

use piston_window::{PistonWindow, Texture, WindowSettings, TextureSettings, clear};
use image::{
    ConvertBuffer,
    ImageBuffer,
    Rgba
};

use std::net::UdpSocket;

use std::sync::mpsc::{Receiver, Sender, channel};

pub mod local;
pub mod network;
pub mod remote;
pub mod transcode;

fn main() {
    let local_address = network::connect::init("stun.stunprotocol.org:3478", 34254);
    //TODO(teo): error handling
    println!("Others can connect to you at: {:?}", local_address);
    let local_thread = std::thread::Builder::new().name("Local Window".to_string()).spawn(move || {
        local::capture::main()
    });
    let remote_thread = std::thread::Builder::new().name("Remote Window".to_string()).spawn(move || {
        remote::capture::main();
    });
    local_thread.unwrap().join().unwrap();
    remote_thread.unwrap().join().unwrap();
    /*
    let mut window: PistonWindow =
        WindowSettings::new("You", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut remote_window: PistonWindow =
        WindowSettings::new("Them", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut tex: Option<Texture<_>> = None;
    let mut remote_tex: Option<Texture<_>> = None;

    //webcam to window
    let (sender, receiver) = std::sync::mpsc::channel();

    //webcam to network
    let (netsender, netreceiver): (_, Receiver<Vec<u8>>)= std::sync::mpsc::channel();

    //network to window
    let (remotesender, remotereceiver) = std::sync::mpsc::channel();

    //webcam to network
    let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");

    //network to window
    let remotesocket = UdpSocket::bind("127.0.0.1:4242").expect("couldn't bind to address");

    let remotethread = std::thread::spawn(move || {
        //let mut frame_buf = vec![0; 1228800];
        let mut frame_buf: Vec<u8> = Vec::with_capacity(1228800);
        loop {
            let mut buf = [0; 1500];
            match remotesocket.recv(&mut buf) {
                Ok(received) => {
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
                    remotesender.send(frame);*/
                    println!("chunk #{:?}", buf[0]);
                    frame_buf.extend(buf.iter());
                    if frame_buf.len() >= 1228800 as usize {
                        println!("we got a frame fam");
                        let mut new_frame = frame_buf.split_off(1228800);
                        let frame: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(640, 480, frame_buf).unwrap();
                        remotesender.send(frame);
                        frame_buf = new_frame;
                    }
                }
                Err(e) => {
                    println!("error reading from remote socket");
                }
            }
        }
    });

    let imgthread = std::thread::spawn(move || {
        let mut cam = camera_capture::create(0).unwrap()
                                                    .fps(30.0)
                                                    .unwrap()
                                                    .start()
                                                    .unwrap();
        loop {
            match cam.next() {
                Some(img) => {
                    //print!("frame");
                    sender.send(img.convert());
                }

                None => {
                    //print!("nothing yet");
                }
            }
        }
    });

    let netthread = std::thread::spawn(move || {
        loop {
            if let Ok(bytes) = netreceiver.try_recv() {
                //let bytes_iter = bytes.iter(); 
                for (i, chunk) in bytes.chunks(1500).enumerate() {
                    let s = [i as u8];
                    let concatenated = [&s, chunk].concat();
                    socket.send_to(chunk, "127.0.0.1:4242").expect("couldn't send data");
                    //print!("sending chunk");
                }

            }
        }
    });

    loop {
        if let Some(e) = window.next() {
            if let Ok(frame) = receiver.try_recv() {
                if let Some(mut t) = tex {
                    t.update(&mut window.encoder, &frame).unwrap();
                    tex = Some(t);
                } else {
                    tex = Texture::from_image(&mut window.factory, &frame, &TextureSettings::new()).ok();
                }
                netsender.send(frame.into_raw());
            }
            window.draw_2d(&e,|c, g| {
                clear([1.0; 4], g);
                if let Some(ref t) = tex {
                    piston_window::image(t, c.transform, g);
                }
            });
        }
        if let Some(e) = remote_window.next() {
            if let Ok(frame) = remotereceiver.try_recv() {
                if let Some(mut t) = remote_tex {
                    t.update(&mut remote_window.encoder, &frame).unwrap();
                    remote_tex = Some(t);
                } else {
                    remote_tex = Texture::from_image(&mut remote_window.factory, &frame, &TextureSettings::new()).ok();
                }
            }
            remote_window.draw_2d(&e,|c, g| {
                clear([1.0; 4], g);
                if let Some(ref t) = remote_tex {
                    piston_window::image(t, c.transform, g);
                }
            });
        }
    }
    drop(receiver);
    //drop(netreceiver);
    imgthread.join().unwrap();
    netthread.join().unwrap();
    remotethread.join().unwrap();
    */
}
