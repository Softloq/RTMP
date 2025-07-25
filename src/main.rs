mod rtmp;

fn main() -> std::io::Result<()> {
	let mut rtmp_server = rtmp::server::RtmpServer::new("127.0.0.1", 1935)?;
	rtmp_server.listen()?;
	Ok(())
}