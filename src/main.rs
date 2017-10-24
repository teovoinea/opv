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
        WindowSettings::new("piston: image", [1280, 720])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut tex: Option<Texture<_>> = None;
    let (sender, receiver) = std::sync::mpsc::channel();

    let (netsender, netreceiver): (_, Receiver<Vec<u8>>)= std::sync::mpsc::channel();

    let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");

    let imgthread = std::thread::spawn(move || {
        let mut cam = camera_capture::create(0).unwrap()
                                                    .fps(30.0)
                                                    .unwrap()
                                                    .start()
                                                    .unwrap();
        loop {
            match cam.next() {
                Some(img) => {
                    print!("frame");
                    sender.send(img.convert());
                }

                None => {
                    print!("nothing yet");
                }
            }
        }
    });

    let netthread = std::thread::spawn(move || {
        loop {
            if let Ok(bytes) = netreceiver.try_recv() {
                //let bytes_iter = bytes.iter(); 
                for chunk in bytes.chunks(100) {
                    socket.send_to(chunk, "127.0.0.1:4242").expect("couldn't send data");
                    print!("sending chunk");
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
}
