// 版权归亚马逊公司及其关联公司所有。保留所有权利。
// SPDX-License-Identifier: Apache-2.0

use btcmnetwork::{connservice::ClientPoolManager,eventqueue::{MessageEvent, MessageEventQueue},imserver, mqserver,wdserver};
use btcmweb::webserver;

use tokio::sync::{mpsc::{Receiver, Sender}, Mutex};


use std::error::Error;
// use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use btcmtools::LOGGER;
use slog::info;


// use std::sync::mpsc::channel;
// use ctrlc;

// 全局变量用于指示服务是否应该继续运行
// static SHOULD_RUN: AtomicBool = AtomicBool::new(true);

// 定义服务的配置结构体
#[allow(dead_code)]
struct Config {
    imserver    : String,
    import      : u16,
    webserver   : String,
    webport     : u16,
    log_level   : slog::Level,
}

impl Config {
    #[allow(dead_code)]
    // 从环境变量中加载配置
    fn load_from_env() -> Self {
        let imserver = std::env::var("IMSERVER").unwrap_or_else(|_| "0.0.0.0".to_string());
        let import = std::env::var("IMPORT").unwrap_or_else(|_| "9563".to_string()).parse().unwrap();
        let webserver = std::env::var("WEBSERVER").unwrap_or_else(|_| "0.0.0.0".to_string());
        let webport = std::env::var("WEBPORT").unwrap_or_else(|_| "1220".to_string()).parse().unwrap();

        let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()).parse().unwrap();
        Config { imserver, import,webserver,webport, log_level }
    }
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // 加载配置
    // let config = Config::load_from_env();


    info!(LOGGER, "start server");

    // let (tx, _rx) = channel();
    
    // ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel.")).expect("Error setting Ctrl-C handler");


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

/*
 * 
 * 
 * 在 Rust 中，编写一个通用的后台服务的程序框架需要考虑到很多因素，比如异步性能、错误处理、日志记录、配置管理等。
 * 下面是一个简单的通用后台服务程序框架示例，它使用了 Tokio 作为异步运行时，并使用 slog-rs 进行日志记录：
 use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use slog::{info, o, Drain};
use slog_async::Async;
use slog_term::{FullFormat, TermDecorator};

// 全局变量用于指示服务是否应该继续运行
static SHOULD_RUN: AtomicBool = AtomicBool::new(true);

// 定义服务的配置结构体
struct Config {
    address: String,
    port: u16,
    log_level: slog::Level,
}

impl Config {
    // 从环境变量中加载配置
    fn load_from_env() -> Self {
        let address = std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap();
        let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()).parse().unwrap();
        Config { address, port, log_level }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 加载配置
    let config = Config::load_from_env();

    // 初始化日志记录器
    let log = init_logger(config.log_level)?;

    // 捕获 SIGINT 信号（Ctrl + C）
    let (stop_signal_sender, stop_signal_receiver) = oneshot::channel();
    ctrlc::set_handler(move || {
        info!(log, "Received SIGINT, stopping server");
        stop_signal_sender.send(()).unwrap();
    })?;

    // 启动服务
    let listener = TcpListener::bind(format!("{}:{}", config.address, config.port)).await?;
    info!(log, "Server listening on {}:{}", config.address, config.port);

    // 处理连接
    while let Ok((socket, _)) = listener.accept().await {
        tokio::spawn(handle_connection(socket, log.clone(), stop_signal_receiver.clone()));
    }

    Ok(())
}

async fn handle_connection(socket: tokio::net::TcpStream, log: slog::Logger, stop_signal_receiver: oneshot::Receiver<()>) {
    // 在这里处理连接
    // 可以使用 log 记录日志
    while SHOULD_RUN.load(Ordering::Relaxed) {
        // 处理连接的代码
    }

    // 停止信号接收器已经收到信号，停止服务
    info!(log, "Stopping server");
    SHOULD_RUN.store(false, Ordering::Relaxed);
}

// 初始化日志记录器
fn init_logger(log_level: slog::Level) -> Result<slog::Logger, Box<dyn Error>> {
    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let drain = Async::new(drain).build().fuse();
    let drain = Mutex::new(drain).fuse();
    Ok(slog::Logger::root(drain, o!()).level(log_level))
}

 */