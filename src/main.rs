mod rtmp;

fn main() -> std::io::Result<()> {
	let mut tcp_server = rtmp::server::Server::new("127.0.0.1", 1935)?;
	tcp_server.listen()?;
	Ok(())
}