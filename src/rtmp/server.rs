use rtmp::connection::{RtmpConnection};
use rtmp::protocol::{rtmp_chunk_stream_protocol};
use crate::rtmp;

use std::net::{TcpListener, TcpStream};
use std::{io, thread};

pub struct RtmpServer { listener: TcpListener }
impl RtmpServer {
	pub fn new(host: &str, port: u16) -> io::Result<Self> {
		let listener: TcpListener = TcpListener::bind(format!("{}:{}", host, port))?;
		Ok(RtmpServer { listener })
	}

	pub fn listen(&mut self) -> io::Result<()> {
		for tcp_stream_attempt in self.listener.incoming() {
			if let Err(e) = tcp_stream_attempt {
 				eprintln!("Error accepting connection: {}", e);
				if e.kind() == io::ErrorKind::AddrInUse || e.kind() == io::ErrorKind::PermissionDenied {
					eprintln!("Fatal listener error, shutting down.");
					return Err(e);
				}
				continue
			}
			
			let tcp_stream = tcp_stream_attempt.unwrap();
            self.handle_connection(tcp_stream);
		}
		Ok(())
	}

	fn handle_connection(&self, stream: TcpStream) {
		thread::spawn(move || {
			let rtmp_conn_attempt = RtmpConnection::new(stream);
			if let Err(e) = rtmp_conn_attempt {
				eprintln!("[RTMP Server] Error creating client: {}", e);
				return
			}

			let rtmp_conn = rtmp_conn_attempt.unwrap();
			println!("[RTMP Server] Client '{}' | Connected. Starting RTMP Protocol on connection.", rtmp_conn.client_ip_addr());

			rtmp_chunk_stream_protocol(rtmp_conn);
		});
	}
}