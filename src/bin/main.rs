// 版权归亚马逊公司及其关联公司所有。保留所有权利。
// SPDX-License-Identifier: Apache-2.0

use btcmnetwork::imserver;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let imserver_result = imserver::start_instant_message_server().await;
    
    imserver_result
}