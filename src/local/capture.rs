extern crate camera_capture;
extern crate piston_window;
extern crate image;
extern crate texture;

use piston_window::{PistonWindow, Texture, WindowSettings, TextureSettings, clear};
use image::{
    ConvertBuffer,
    ImageBuffer,
    Rgba
};

use std;
use network::send;
use transcode::encode;

pub fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("You", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let mut tex: Option<Texture<_>> = None;
    
    let (sender, receiver) = std::sync::mpsc::channel();
    let (netsender, netreceiver) = std::sync::mpsc::channel();

    let localSend = send::net_send("127.0.0.1:34254", "127.0.0.1:4242", netreceiver);
    
    let imgthread = std::thread::Builder::new().name("Webcam".to_string()).spawn(move || {
        let mut cam = camera_capture::create(0).unwrap()
                                                    .fps(30.0)
                                                    .unwrap()
                                                    .start()
                                                    .unwrap();
        loop {
            match cam.next() {
                Some(img) => {
                    //TODO(teo): error handling
                    sender.send(img.convert());
                    netsender.send(encode::frame_to_chunks(img.convert()));
                }

                None => {
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
        }
        window.draw_2d(&e,|c, g| {
            clear([1.0; 4], g);
            if let Some(ref t) = tex {
                piston_window::image(t, c.transform, g);
            }
        });
    }
    drop(receiver);
    imgthread.unwrap().join().unwrap();
}