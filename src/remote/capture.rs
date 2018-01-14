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
use network::recv;

pub fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Them", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut tex: Option<Texture<_>> = None;

    let (netsender, netreceiver) = std::sync::mpsc::channel();
    let localRecv = recv::net_recv("127.0.0.1:4242", netsender);

    while let Some(e) = window.next() {
        if let Ok(frame) = netreceiver.try_recv() {
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
    drop(netreceiver);
}