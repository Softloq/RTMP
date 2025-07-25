use rtmp::connection::{RtmpConnection};
use rtmp::handshake_policy::policy::{rtmp_handshake_policy};
use rtmp::chunk_stream::stream::{RtmpChunkStream};
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

			let mut rtmp_conn = rtmp_conn_attempt.unwrap();
			println!("[RTMP Server] Client '{}' | Connected. Starting RTMP Chunk Stream.", rtmp_conn.client_ip_addr());

			let handshake_attempt = rtmp_handshake_policy(&mut rtmp_conn);
			if let Err(e) = handshake_attempt {
				eprintln!("[RTMP Handshake Error] {}", e); 
				return
			}
			
			loop {
				let mut chunk_stream = RtmpChunkStream::new(rtmp_conn);
				chunk_stream.chunking();
				break
			}
		});
	}
}