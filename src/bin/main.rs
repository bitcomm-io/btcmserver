// 版权归亚马逊公司及其关联公司所有。保留所有权利。
// SPDX-License-Identifier: Apache-2.0

use btcmnetwork::{connservice::ClientPoolManager, imserver};
use btcmweb::webserver;
use std::{error::Error, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cpm0 = Arc::new(tokio::sync::Mutex::new(ClientPoolManager::new()));
    // 使用 tokio::spawn 启动异步任务
    let cpm1 = cpm0.clone();
    // IM Server
    let imserver_handle = tokio::spawn(async move {
        imserver::start_instant_message_server(cpm0).await.expect("imserver error!");
    });
    // Web Server
    let webserver_handle = tokio::spawn(async move {
        webserver::star_webserver(cpm1).await;
    });
    // 等待两个服务执行完毕
    // #[allow(unused_variables)]
    tokio::try_join!(imserver_handle,webserver_handle)?;

    Ok(())
}