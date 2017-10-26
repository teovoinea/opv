extern crate camera_capture;
extern crate piston_window;
extern crate image;
extern crate texture;


use piston_window::{PistonWindow, Texture, WindowSettings, TextureSettings, clear};
use image::ConvertBuffer;

use std::net::UdpSocket;

use std::sync::mpsc::{Receiver, Sender, channel};

fn main() {
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

    //webcam to window
    let (sender, receiver) = std::sync::mpsc::channel();

    //webcam to network
    let (netsender, netreceiver): (_, Receiver<Vec<u8>>)= std::sync::mpsc::channel();

    //network to window
    //let (remotesender, remotereceiver) = std::sync::mpsc::channel();

    //webcam to network
    let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");

    //network to window
    let remotesocket = UdpSocket::bind("127.0.0.1:4242").expect("couldn't bind to address");

    let remotethread = std::thread::spawn(move || {
        loop {
            let mut buf = [0; 1500];
            match remotesocket.recv(&mut buf) {
                Ok(received) => {
                    //received this many bytes
                    println!("{:?}", received);
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
                for chunk in bytes.chunks(1500) {
                    socket.send_to(chunk, "127.0.0.1:4242").expect("couldn't send data");
                    //print!("sending chunk");
                }

            }
        }
    });

    while let Some(e) = window.next() {
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
    drop(receiver);
    //drop(netreceiver);
    imgthread.join().unwrap();
    netthread.join().unwrap();
    remotethread.join().unwrap();
}
