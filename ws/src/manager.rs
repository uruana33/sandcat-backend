use crate::client::Client;
use abi::errors::Error;
use abi::message::chat_service_client::ChatServiceClient;
use abi::message::msg::Data;
use abi::message::{Msg, MsgResponse, SendMsgRequest};
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::transport::Channel;
use tracing::{debug, error, info};

type UserID = String;
type PlatformID = String;
/// client hub
type Hub = Arc<DashMap<UserID, DashMap<PlatformID, Client>>>;

/// manage the client
#[derive(Clone)]
pub struct Manager {
    tx: mpsc::Sender<Msg>,
    pub hub: Hub,
    pub redis: redis::Client,
    pub chat_rpc: ChatServiceClient<Channel>,
}

#[allow(dead_code)]
impl Manager {
    pub fn new(
        tx: mpsc::Sender<Msg>,
        redis: redis::Client,
        msg_rpc: ChatServiceClient<Channel>,
    ) -> Self {
        Manager {
            tx,
            hub: Arc::new(DashMap::new()),
            redis,
            chat_rpc: msg_rpc,
        }
    }

    pub async fn send_msg(&self, msg: Msg) {
        if msg.data.is_none() {
            return;
        }
        let data = msg.data.as_ref().unwrap();
        match data {
            Data::Single(_) => {
                self.send_single_msg(&msg.receiver_id, &msg).await;
            }
            Data::Group(_) => {
                // todo think about how to deal with group message,
                // shall we need to query members id from database?
                // or is there another better way?
                self.send_group(&vec![msg.receiver_id.clone()], &msg).await;
            }
            // ignore server response type
            Data::Response(_) => {}
        }
    }
    pub async fn send_group(&self, obj_ids: &Vec<String>, msg: &Msg) {
        for id in obj_ids {
            if let Some(clients) = self.hub.get(id) {
                self.send_msg_to_clients(&clients, msg).await;
            }
        }
    }

    pub async fn send_single_msg(&self, obj_id: &str, msg: &Msg) {
        if let Some(clients) = self.hub.get(obj_id) {
            self.send_msg_to_clients(&clients, msg).await;
        }
    }

    async fn send_msg_to_clients(&self, clients: &DashMap<PlatformID, Client>, msg: &Msg) {
        for client in clients.iter() {
            let content = serde_json::to_string(&msg).expect("序列化出错");
            if let Err(e) = client.value().send_text(content).await {
                error!("msg send error: {:?}", e);
            } else {
                // debug!("消息发送成功--{:?}", client.id.clone());
            }
        }
    }

    // 注册客户端
    // todo check platform id, if existed already, kick offline
    pub async fn register(&mut self, id: String, client: Client) {
        if let Some(cli) = self.hub.get_mut(&id) {
            cli.insert(client.platform_id.clone(), client);
        } else {
            let hash_map = DashMap::new();
            hash_map.insert(client.user_id.clone(), client);
            self.hub.insert(id, hash_map);
        }
    }

    // 删除客户端
    pub async fn unregister(&mut self, id: String, printer_id: String) {
        if let Some(clients) = self.hub.get_mut(&id) {
            if clients.len() == 1 {
                self.hub.remove(&id);
            } else {
                clients.remove(&printer_id);
            }
        }
    }

    pub async fn run(&mut self, mut receiver: mpsc::Receiver<Msg>) {
        info!("manager start");
        // 循环读取消息
        while let Some(mut message) = receiver.recv().await {
            // request the message rpc to get server_msg_id
            debug!("receive message: {:?}", message);
            match self
                .chat_rpc
                .send_msg(SendMsgRequest {
                    message: Some(message.clone()),
                })
                .await
            {
                Ok(res) => {
                    // reply send success
                    let response = res.into_inner();
                    if response.err.is_empty() {
                        debug!("send message success");
                    } else {
                        error!("send message error: {:?}", response.err);
                    }
                    message.server_id = response.server_id.clone();
                    message.data = Some(Data::Response(response));
                }
                Err(err) => {
                    error!("send message error: {:?}", err);
                    let response = MsgResponse::from(err);
                    message.data = Some(Data::Response(response));
                }
            }

            // reply result to sender
            debug!("reply message:{:?}", message);
            self.send_single_msg(&message.send_id, &message).await;
        }
    }

    pub async fn broadcast(&self, msg: Msg) -> Result<(), Error> {
        self.tx.send(msg).await.map_err(|_| Error::BroadCastError)
    }
}
