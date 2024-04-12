use crate::client::Client;
use abi::config::Config;
use abi::errors::Error;
use abi::message::chat_service_client::ChatServiceClient;
use abi::message::{ContentType, Msg, MsgResponse, MsgType, SendMsgRequest};
use cache::Cache;
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
    pub cache: Box<dyn Cache>,
    pub chat_rpc: ChatServiceClient<Channel>,
}

#[allow(dead_code)]
impl Manager {
    pub async fn new(tx: mpsc::Sender<Msg>, config: &Config) -> Self {
        let cache = cache::cache(config);
        let chat_rpc = Self::get_chat_rpc_client(config)
            .await
            .expect("chat rpc can't open");
        Manager {
            tx,
            hub: Arc::new(DashMap::new()),
            cache,
            chat_rpc,
        }
    }

    async fn get_chat_rpc_client(config: &Config) -> Result<ChatServiceClient<Channel>, Error> {
        // use service register center to get ws rpc url
        let channel = utils::get_rpc_channel_by_name(
            config,
            &config.rpc.chat.name,
            &config.rpc.chat.protocol,
        )
        .await?;
        let chat_rpc = ChatServiceClient::new(channel);
        Ok(chat_rpc)
    }

    pub async fn send_group(&self, obj_ids: &Vec<String>, mut msg: Msg) {
        for id in obj_ids {
            if let Some(clients) = self.hub.get(id) {
                // need to query the users seq
                let seq = match self.cache.get_seq(id).await {
                    Ok(seq) => seq,
                    Err(e) => {
                        error!("get seq error: {:?}", e);
                        continue;
                    }
                };
                msg.seq = seq;
                self.send_msg_to_clients(&clients, &msg).await;
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
            // let content = serde_json::to_string(&msg).expect("序列化出错");
            // if let Err(e) = client.value().send_text(content).await {
            let content = match bincode::serialize(msg) {
                Ok(res) => res,
                Err(_) => {
                    error!("msg serialize error");
                    return;
                }
            };
            if let Err(e) = client.value().send_binary(content).await {
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
            let dash_map = DashMap::new();
            dash_map.insert(client.user_id.clone(), client);
            self.hub.insert(id, dash_map);
        }
    }

    // 删除客户端
    pub async fn unregister(&mut self, id: String, printer_id: String) {
        let mut flag = false;
        if let Some(clients) = self.hub.get_mut(&id) {
            if clients.len() == 1 {
                flag = true;
            } else {
                clients.remove(&printer_id);
            }
        };
        if flag {
            self.hub.remove(&id);
        }
        debug!("unregister client: {:?}", id);
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
                        message.content_type = ContentType::Error as i32;
                    }
                    message.msg_type = MsgType::MsgRecResp as i32;
                    message.server_id = response.server_id.clone();
                    message.content = response.err.into_bytes();
                }
                Err(err) => {
                    error!("send message error: {:?}", err);
                    let response = MsgResponse::from(err);
                    message.content = response.err.into_bytes();
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