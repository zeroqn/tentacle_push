mod push_bytes;

use futures::stream::StreamExt;
use tentacle::{builder::ServiceBuilder, secio::SecioKeyPair, service::TargetProtocol};

#[tokio::main]
async fn main() {
    env_logger::init();

    let push_size = std::env::args().nth(2).map(|s| s.parse::<usize>().ok()).flatten();
    let push_bytes = push_bytes::PushBytes::new(1.into(), push_size);
    let mut service = ServiceBuilder::default().insert_protocol(push_bytes)
        .key_pair(SecioKeyPair::secp256k1_generated())
        .build(());

    if std::env::args().nth(1) == Some("server".to_string()) {
        if push_size.is_none() {
            panic!("server push size is none");
        }

        service.listen("/ip4/0.0.0.0/tcp/2233".parse().expect("listen address")).await.expect("listen");

        loop {
            if service.next().await.is_none() {
                break;
            }
        }
    } else {
        let server = {
            let server = std::env::args().nth(1).expect("missing server ip");
            format!("/ip4/{}/tcp/2233", server).parse().expect("server address")
        };

        service.dial(server, TargetProtocol::All).await.expect("dial");

        loop {
            if service.next().await.is_none() {
                break;
            }
        }
    }
}
