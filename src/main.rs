mod tcp;

fn main() -> std::io::Result<()> {
	let tcp_server = tcp::server::Server::new("127.0.0.1", 1935)?;
	tcp_server.listen()?;
	Ok(())
}