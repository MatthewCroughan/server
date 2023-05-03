use super::client::Client;
use super::task;
use color_eyre::eyre::Result;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::UnixListener;
use tokio::task::JoinHandle;
use tracing::error;

pub struct EventLoop {
	join_handle: JoinHandle<()>,
}

impl EventLoop {
	pub fn new(socket_path: PathBuf) -> Result<Arc<Self>> {
		let socket = UnixListener::bind(socket_path)?;

		let join_handle = task::new(|| "event loop", async move {
			loop {
				let Ok((socket, _)) = socket.accept().await else { continue };
				if let Err(e) = Client::from_connection(socket) {
					error!(?e, "Unable to create client from connection");
				}
			}
		})?;
		let event_loop = Arc::new(EventLoop { join_handle });

		Ok(event_loop)
	}
}

impl Drop for EventLoop {
	fn drop(&mut self) {
		self.join_handle.abort();
	}
}
