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
#[macro_use]
extern crate clap;
extern crate pnet;

use clap::{Arg, App, SubCommand};
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
    let matches = App::new("OPV")
                        .version("0.1")
                        .author("Teo V. <voinea.teodor@gmail.com>")
                        .about("Open source peer to peer video chat")
                        .arg(Arg::with_name(("source ip"))
                            .short("a")
                            .long("source-ip")
                            .value_name("<source-ip")
                            .help("Sets the local networking interface")
                            .takes_value(true)
                            .default_value("auto"))
                        .arg(Arg::with_name("source port")
                            .short("p")
                            .long("source-port")
                            .value_name("<source-port>")
                            .help("Sets the sending port")
                            .takes_value(true)
                            .default_value("34254"))
                        .arg(Arg::with_name("listening port")
                            .short("l")
                            .long("listening-port")
                            .value_name("<listening-port>")
                            .help("Sets the listening port")
                            .takes_value(true)
                            .default_value("4242"))
                        .arg(Arg::with_name("STUN address")
                            .short("s")
                            .long("stun-address")
                            .value_name("<stun.server.address>:<port-number>")
                            .help("Sets the stun server. eg: stun.mystunserver.org:3478")
                            .takes_value(true)
                            .default_value("stun.stunprotocol.org:3478"))
                        .arg(Arg::with_name("destination ip")
                            .short("d")
                            .long("destination-ip")
                            .value_name("<destination-ip>")
                            .help("Sets the ip of the other party")
                            .takes_value(true)
                            .required(true))
                        .arg(Arg::with_name("destination port")
                            .short("i")
                            .long("destination-port")
                            .value_name("<destionation-port>")
                            .help("Sets the port of the other party")
                            .takes_value(true)
                            .default_value("4242"))
                        .get_matches();

    let source_port = value_t!(matches, "source port", u16).unwrap_or(34254);
    let listening_port = value_t!(matches, "listening port", u16).unwrap_or(34254);
    let mut source_ip = matches.value_of("source ip").unwrap().to_string();
    if source_ip == "auto" {
        //source_address = network::connect::find_interface();
        source_ip = String::from("192.168.10.102");
    }
    let mut source_address = String::new();
    source_address.push_str(&source_ip);
    source_address.push_str(":");
    source_address.push_str(&source_port.to_string());
    let mut listening_address = String::new();
    listening_address.push_str(&source_ip);
    listening_address.push_str(":");
    listening_address.push_str(&listening_port.to_string());
    let stun_server = matches.value_of("STUN address").unwrap();
    let destination_ip = matches.value_of("destination ip").unwrap();
    let destination_port = matches.value_of("destination port").unwrap();
    let mut destination_address = String::new();
    destination_address.push_str(&destination_ip);
    destination_address.push_str(":");
    destination_address.push_str(&destination_port);

    let internet_address = network::connect::init(stun_server, listening_port);

    println!("Sending from {:?} to {:?}", source_address, destination_address);
    println!("Listening on {:?}", listening_address);
    //TODO(teo): error handling
    println!("Others can connect to you at: {:?}", internet_address);
    let local_thread = std::thread::Builder::new().name("Local Window".to_string()).spawn(move || {
        local::capture::main(source_address.as_str(), destination_address.as_str());
    });
    let remote_thread = std::thread::Builder::new().name("Remote Window".to_string()).spawn(move || {
        remote::capture::main(listening_address.as_str());
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
