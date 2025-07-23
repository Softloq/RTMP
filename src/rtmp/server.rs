use rtmp::stream::{Stream};
use rtmp::handshake::{handshake};

use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;
use std::{io, thread};

use crate::rtmp;

pub struct Server {
	listener: TcpListener,
	clients: HashMap<String, Stream>
}

impl Server {
	pub fn new(host: &str, port: u16) -> io::Result<Self> {
		let listener: TcpListener = TcpListener::bind(format!("{}:{}", host, port))?;
		let clients: HashMap<String, Stream> = HashMap::new();
		Ok(Server { listener, clients })
	}

	pub fn listen(&mut self) -> io::Result<()> {
		for stream in self.listener.incoming() {
		    match stream {
                Ok(stream) => { self.new_conn(stream); }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                    if e.kind() == io::ErrorKind::AddrInUse || e.kind() == io::ErrorKind::PermissionDenied {
                        eprintln!("Fatal listener error, shutting down.");
                        return Err(e);
                    }
                }
            }
		}
		Ok(())
	}

	fn new_conn(&self, stream: TcpStream) {
		thread::spawn(|| {
			let client: Result<Stream, io::Error> = Stream::new(stream);
			match client {
				Ok(mut client) => {
					println!("[RTMP Server] Client '{}' | Connected.", client.ip_addr());

					match handshake(&mut client) {
						Ok(()) => {

						}
						Err(e) => {
							eprintln!("[RTMP Handshake Error] {}", e);
						}
					}
				}
				Err(e) => {
					eprintln!("Error creating client: {}", e);
				}
			}
		});
	}
}