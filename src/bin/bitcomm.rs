// 版权归亚马逊公司及其关联公司所有。保留所有权利。
// SPDX-License-Identifier: Apache-2.0

use btcmnetwork::{connservice::ClientPoolManager,eventqueue::{MessageEvent, MessageEventQueue},imserver, mqserver,wdserver};
use btcmweb::webserver;

use tokio::sync::{mpsc::{Receiver, Sender}, Mutex};


use std::error::Error;
// use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::mpsc::channel;
use ctrlc;
use btcmtools::{LOGGER,BitcommOpt};
use slog::info;
use tokio::signal;

use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 解析命令行参数
    let opt = BitcommOpt::from_args();
    // 初始化事件队列
    let meq = MessageEventQueue::new();
    // 初始化事件收发器
    let meqsend: Arc<Mutex<Sender<MessageEvent>>> = Arc::new(Mutex::new(meq.sender));
    let meqrece = Arc::new(Mutex::new(meq.receiver));
    // 创建客户端池管理器
    let cpm = Arc::new(Mutex::new(ClientPoolManager::new()));
    // 初始化Ctrl+C处理过程
    // init_citric_system();
    // 
    switch_command(opt,&cpm,&meqsend,&meqrece).await
}

fn _init_citric_system() {
    let (tx, rx) = channel();
    
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");
    
    tokio::spawn(async move {
        println!("Waiting for Ctrl-C...");
        rx.recv().expect("Could not receive from channel.");
        println!("Got it! Exiting...");
    });
}

async fn switch_command(
    cmdopt:BitcommOpt,
    cpm:     &Arc<Mutex<ClientPoolManager>>,
    meqsend: &Arc<Mutex<Sender<MessageEvent>>>,
    meqrece: &Arc<Mutex<Receiver<MessageEvent>>>) -> Result<(), Box<dyn Error>> {

    match cmdopt {
        BitcommOpt::StartServer => {
            return start_server(cpm,meqsend,meqrece).await;
        }
        BitcommOpt::StopServer => {
            stop_server();
        }
    }
    Ok(())
}
 
fn stop_server() {
    let pid = btcmtools::pid::read_pid();
    if pid != -1 {
        btcmtools::pid::kill_pid(pid);
    } else {
        info!(LOGGER,"Service not started");
    }   
    btcmtools::pid::dele_pid();
}

async fn start_server(
    cpm:     &Arc<Mutex<ClientPoolManager>>,
    meqsend: &Arc<Mutex<Sender<MessageEvent>>>,
    meqrece: &Arc<Mutex<Receiver<MessageEvent>>>
    ) -> Result<(), Box<dyn Error>> {
    // 写入PID
    btcmtools::pid::save_pid();
    // 输出日志
    info!(LOGGER, "start server...");

    // MQ Server
    let mqserver_handle = get_mqserver_handle(cpm, meqrece);

    // IM Server
    let imserver_handle = get_imserver_handle(cpm, meqsend);

    // Web Server
    let webserver_handle = get_webserver_handle(cpm);

    // WD Server
    let wdserver_handle = get_wdserver_handle(cpm);

    // 等待四个服务执行完毕
    tokio::try_join!(mqserver_handle, imserver_handle, webserver_handle, wdserver_handle)?;
    Ok(())
}

fn get_wdserver_handle(cpm: &Arc<Mutex<ClientPoolManager>>) -> tokio::task::JoinHandle<()> {
    let wdserver_handle = {
        let cpm = cpm.clone();
        let mut sig_int  = signal::unix::signal(signal::unix::SignalKind::interrupt()).unwrap();
        let mut sig_term = signal::unix::signal(signal::unix::SignalKind::terminate()).unwrap();    
        tokio::spawn(async move {
            tokio::select! {
                _ = async {
                    // 等待中断信号
                    sig_int.recv().await;
                } => {
                    // info!(LOGGER,"Received SIGINT, watch dog server shutting down...");
                }
                _ = async {
                    // 等待终止信号
                    sig_term.recv().await;
                } => {
                    // println!("Received SIGTERM, shutting down...");
                }
                _ = async {
                    info!(LOGGER,"Watch Dog Server starting...");
                    // 启动Watch Dog异步任务
                    wdserver::start_watch_dog_server(cpm).await.expect("wdserver error!");
                    // 
                } => {
                    // println!("Received connection, shutting down...");
                }
            }
            info!(LOGGER,"Received SIGINT/SIGTERM, Watch Dog server shutting down...");
        })
    };
    wdserver_handle
}

fn get_webserver_handle(cpm: &Arc<Mutex<ClientPoolManager>>) -> tokio::task::JoinHandle<()> {
    let webserver_handle = {
        let cpm = cpm.clone();
        let mut sig_int  = signal::unix::signal(signal::unix::SignalKind::interrupt()).unwrap();
        let mut sig_term = signal::unix::signal(signal::unix::SignalKind::terminate()).unwrap();    
        tokio::spawn(async move {
            tokio::select! {
                _ = async {
                    // 等待中断信号
                    sig_int.recv().await;
                } => {
                    // println!("Received SIGINT, shutting down...");
                }
                _ = async {
                    // 等待终止信号
                    sig_term.recv().await;
                } => {
                    // println!("Received SIGTERM, shutting down...");
                }
                _ = async {
                    info!(LOGGER,"Web Admin Server starting...");
                    // 启动Watch Dog异步任务
                    webserver::star_webserver(cpm).await;
                    // 
                } => {
                    // println!("Received connection, shutting down...");
                }
            }
            info!(LOGGER,"Received SIGINT/SIGTERM, Web Admin Server shutting down...");
        })
    };
    webserver_handle
}

fn get_imserver_handle(cpm: &Arc<Mutex<ClientPoolManager>>, meqsend: &Arc<Mutex<Sender<MessageEvent>>>) -> tokio::task::JoinHandle<()> {
    let imserver_handle = {
        let cpm = cpm.clone();
        let meqsend = meqsend.clone();
        let mut sig_int  = signal::unix::signal(signal::unix::SignalKind::interrupt()).unwrap();
        let mut sig_term = signal::unix::signal(signal::unix::SignalKind::terminate()).unwrap();    
        tokio::spawn(async move {
            tokio::select! {
                _ = async {
                    // 等待中断信号
                    sig_int.recv().await;
                } => {
                    // println!("Received SIGINT, shutting down...");
                }
                _ = async {
                    // 等待终止信号
                    sig_term.recv().await;
                } => {
                    // println!("Received SIGTERM, shutting down...");
                }
                _ = async {
                    info!(LOGGER,"Intant Message Server starting...");
                    // 启动Watch Dog异步任务
                    imserver::start_instant_message_server(cpm, meqsend).await.expect("imserver error!");
                    // 
                } => {
                    // println!("Received connection, shutting down...");
                }
            }
            info!(LOGGER,"Received SIGINT/SIGTERM, Intant Message Server shutting down...");
        })
    };
    imserver_handle
}

fn get_mqserver_handle(cpm: &Arc<Mutex<ClientPoolManager>>, meqrece: &Arc<Mutex<Receiver<MessageEvent>>>) -> tokio::task::JoinHandle<()> {
    let mqserver_handle = {
        let cpm = cpm.clone();
        let meqrece = meqrece.clone();
        let mut sig_int  = signal::unix::signal(signal::unix::SignalKind::interrupt()).unwrap();
        let mut sig_term = signal::unix::signal(signal::unix::SignalKind::terminate()).unwrap();    
        tokio::spawn(async move {
            tokio::select! {
                _ = async {
                    // 等待中断信号
                    sig_int.recv().await;
                } => {
                    // println!("Received SIGINT, shutting down...");
                }
                _ = async {
                    // 等待终止信号
                    sig_term.recv().await;
                } => {
                    // println!("Received SIGTERM, shutting down...");
                }
                _ = async {
                    // 启动Watch Dog异步任务
                    info!(LOGGER,"Message Queue Server starting...");
                    mqserver::start_message_evnet_queue_server(cpm, meqrece).await.expect("mqserver error!");
                    // 
                } => {
                    println!("Received connection, shutting down...");
                }
            }
            info!(LOGGER,"Received SIGINT/SIGTERM, Message Queue Server shutting down...");
        })
    };
    mqserver_handle
}