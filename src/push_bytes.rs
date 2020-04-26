use bytes::Bytes;
use std::time::SystemTime;
use tentacle::{
    builder::MetaBuilder,
    context::ProtocolContextMutRef,
    service::{ProtocolHandle, ProtocolMeta},
    traits::SessionProtocol,
    ProtocolId,
};

pub const NAME: &str = "push_bytes";
pub const SUPPORT_VERSIONS: [&str; 1] = ["0.1"];

macro_rules! support_versions {
    ($versions:expr) => {
        $versions.to_vec().into_iter().map(String::from).collect()
    };
}

pub struct PushBytes {
    size: Option<usize>,
}

impl PushBytes {
    pub fn new(protocol_id: ProtocolId, size: Option<usize>) -> ProtocolMeta {
        MetaBuilder::new()
            .id(protocol_id)
            .name(|proto_id| format!("{}/{}", NAME, proto_id))
            .support_versions(support_versions!(SUPPORT_VERSIONS))
            .session_handle(move || {
                let handle = Box::new(PushBytes { size });
                ProtocolHandle::Callback(handle)
            })
            .build()
    }
}

impl SessionProtocol for PushBytes {
    fn connected(&mut self, ctx: ProtocolContextMutRef, _version: &str) {
        if ctx.session.ty.is_outbound() {
            // Only push bytes to inbound request
            return;
        }

        let data = b"1".repeat(self.size.expect("push size"));
        if let Err(e) = ctx.send_message(Bytes::copy_from_slice(&data)) {
            log::error!("send to {} failed {}", ctx.session.address, e)
        }
        println!("send bytes to {} at {}", ctx.session.address, now());
    }

    fn received(&mut self, _ctx: ProtocolContextMutRef, data: Bytes) {
        println!("received {} bytes at {}", data.len(), now());
    }
}

fn now() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("should also be positive")
        .as_millis()
}
