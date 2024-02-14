// 版权归亚马逊公司及其关联公司所有。保留所有权利。
// SPDX-License-Identifier: Apache-2.0

use btcmnetwork::{connservice::ClientPoolManager,eventqueue::{MessageEvent, MessageEventQueue},imserver, mqserver,wdserver};
use btcmweb::webserver;
use tokio::sync::{mpsc::{Receiver, Sender}, Mutex};
use std::{error::Error, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {


    // 加载配置
    let config = Config::load_from_env();


    let meq = MessageEventQueue::new();
    let meqsend: Arc<Mutex<Sender<MessageEvent>>>   = Arc::new(Mutex::new(meq.sender));
    let meqrece: Arc<Mutex<Receiver<MessageEvent>>> = Arc::new(Mutex::new(meq.receiver));
    let cpm0 = Arc::new(tokio::sync::Mutex::new(ClientPoolManager::new()));
    // 使用 tokio::spawn 启动异步任务
    // MQ Server
    let cpm1 = cpm0.clone();
    let mqserver_handle = tokio::spawn(async move {
        mqserver::start_message_evnet_queue_server(cpm1, meqrece).await.expect("mqserver error!");
    });
    // IM Server
    let cpm2 = cpm0.clone();
    let imserver_handle = tokio::spawn(async move {
        imserver::start_instant_message_server(cpm2,meqsend).await.expect("imserver error!");
    });
    // Web Server
    let cpm3 = cpm0.clone();
    let webserver_handle = tokio::spawn(async move {
        webserver::star_webserver(cpm3).await;
    });
    // WD Server
    let cpm4 = cpm0.clone();
    let wdserver_handle = tokio::spawn(async move {
        wdserver::start_watch_dog_server(cpm4).await.expect("wdserver error!");
    });
    // 等待四个服务执行完毕
    tokio::try_join!(mqserver_handle,imserver_handle,webserver_handle,wdserver_handle)?;

    Ok(())
} 