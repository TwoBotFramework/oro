use oro_core::{EventHandlerRegister, Message};

#[tokio::main]
async fn main() {
    let mut cli = oro_core::Client::new(oro_core::Config {
        host: "10.147.18.50".to_owned(),
        port: 3001,
        token: None,
    });

    cli.register(|event: Box<oro_core::GroupMsg>, api| async move {
        if event.raw_message.contains("测试") {
            api.send_group_msg(event.group_id, vec![Message::Face { id: 1 }])
                .await
                .unwrap();
        }
    });

    cli.start().await.unwrap();
}
