use std::time::Duration;

#[derive(Copy, Clone, Debug)]
pub struct ProtocolConfiguration {
	pub disconnect_timeout: Duration,
}
