// 版权归亚马逊公司及其关联公司所有。保留所有权利。
// SPDX-License-Identifier: Apache-2.0

// 导入相关模块和库
use btcmnetwork::{ imserver, mqserver, wdserver };
use btcmweb::webserver;
use colored::Colorize;
use std::error::Error;
use std::sync::mpsc::channel;
use ctrlc;
use btcmtools::BitcommOpt;
// use slog::info;
use tokio::signal;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


/// 主函数，程序入口
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    init_tracing();
    print_log();
    // let version: &'static str = env!("CARGO_PKG_VERSION");
    // info!("bitcomm version = {}", env!("CARGO_PKG_VERSION"));

    return start_server().await;
    // 解析命令行参数
    // let opt = BitcommOpt::from_args();
    //
    // switch_command(opt).await
}

fn print_log() {
    println!("");
    println!("{}","                              ( * )".blue());
    println!("{}","                            /   |   \\ ".blue()); 
    println!("{}","                      ( * ) ----|---- ( * )".blue());
    println!("{}","                    /   |   \\   |   /   |   \\".blue());
    println!("{}","              ( * ) ----|---- ( * ) ----|---- ( * )".blue());
    println!("{}","            /   |   \\   |   /   |   \\   |   /   |   \\ ".blue());
    println!("{}","      ( * ) ----|---- ( * ) ----|---- ( * ) ----|---- ( * )".blue());
    println!("{}","        |   \\   |   /   |   \\   |   /   |   \\   |   /   |".blue());
    println!("{}","        |---- ( * ) ----|---- ( * ) ----|---- ( * ) ----|".blue());
    println!("{}","        |   /   |   \\   |   /   |   \\   |   /   |   \\   | ".blue());
    println!("{}","      ( * ) ----|---- ( * ) ----|---- ( * ) ----|---- ( * )  ".blue());
    println!("{}{}","            \\   |   /   |   \\   |   /   |   \\   |   /".blue(),"         Welcome to Bitcomm ".red());
    println!("{}{}{}","              ( * ) ----|---- ( * ) ----|---- ( * )".blue(),"             bitcomm version ".red(), env!("CARGO_PKG_VERSION").yellow());
    println!("{}{}","            /   |   \\   |   /   |   \\   |   /   |   \\".blue(),"         Http2/3,Quic,Redis,...".red());
    println!("{}","      ( * ) ----|---- ( * ) ----|---- ( * ) ----|---- ( * )".blue());
    println!("{}","        |   \\   |   /   |   \\   |   /   |   \\   |   /   |".blue());
    println!("{}","        |---- ( * ) ----|---- ( * ) ----|---- ( * ) ----|".blue());
    println!("{}","        |   /   |   \\   |   /   |   \\   |   /   |   \\   | ".blue());
    println!("{}","      ( * ) ----|---- ( * ) ----|---- ( * ) ----|---- ( * )".blue());
    println!("{}","            \\   |   /   |   \\   |   /   |   \\   |   / ".blue());
    println!("{}","              ( * ) ----|---- ( * ) ----|---- ( * )  ".blue());
    println!("{}","                    \\   |   /   |   \\   |   /".blue());
    println!("{}","                      ( * ) ----|---- ( * )".blue());
    println!("{}","                            \\   |   /".blue());
    println!("{}","                              ( * )".blue());
    println!("{}","                         bitcomm server".green());
    println!("{}","                   decentralized communication".green());
    println!("");
}
fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,jwt_authorizer=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}
/// 初始化 Citric 系统，设置 Ctrl-C 信号处理
fn _init_citric_system() {
    let (tx, rx) = channel();

    ctrlc
        ::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    tokio::spawn(async move {
        println!("Waiting for Ctrl-C...");
        rx.recv().expect("Could not receive from channel.");
        println!("Got it! Exiting...");
    });
}

/// 异步函数，根据命令行参数执行相应操作
#[allow(dead_code)]
async fn switch_command(cmdopt: BitcommOpt) -> Result<(), Box<dyn Error>> {
    match cmdopt {
        BitcommOpt::StartServer => {
            return start_server().await;
        }
        BitcommOpt::StopServer => {
            stop_server();
        }
    }
    Ok(())
}

/// 停止服务器，通过读取 PID 并发送信号给进程
fn stop_server() {
    let pid = btcmtools::pid::read_pid();
    if pid != -1 {
        btcmtools::pid::kill_pid(pid);
    } else {
        info!( "Service not started");
    }
    btcmtools::pid::dele_pid();
}

/// 启动服务器，包括获取 MQ Server、IM Server、Web Server 和 WD Server 异步任务的句柄
async fn start_server() -> Result<(), Box<dyn Error>> {
    // 写入 PID
    btcmtools::pid::save_pid();
    // 输出日志
    info!("start server...");

    // 获取 MQ Server 异步任务句柄
    let mqserver_handle = get_mqserver_handle();

    // 获取 IM Server 异步任务句柄
    let imserver_handle = get_imserver_handle();

    // 获取 Web Server 异步任务句柄
    let webserver_handle = get_webserver_handle();

    // 获取 WD Server 异步任务句柄
    let wdserver_handle = get_wdserver_handle();

    // 等待四个服务执行完毕
    tokio::try_join!(mqserver_handle, imserver_handle, webserver_handle, wdserver_handle)?;

    Ok(())
}

/// 获取 Watch Dog Server 异步任务句柄
fn get_wdserver_handle() -> tokio::task::JoinHandle<()> {
    let wdserver_handle = {
        let mut sig_int = signal::unix::signal(signal::unix::SignalKind::interrupt()).unwrap();
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
                    info!("Watch Dog Server starting...");
                    // 启动 Watch Dog 异步任务
                    wdserver::start_watch_dog_server().await.expect("wdserver error!");
                    // 
                } => {
                    // println!("Received connection, shutting down...");
                }
            }

            info!("Received SIGINT/SIGTERM, Watch Dog server shutting down...");
        })
    };
    wdserver_handle
}

/// 获取 Web Admin Server 异步任务句柄
fn get_webserver_handle() -> tokio::task::JoinHandle<()> {
    let webserver_handle = {
        let mut sig_int = signal::unix::signal(signal::unix::SignalKind::interrupt()).unwrap();
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
                    info!("Web Admin Server starting...");
                    // 启动 Web Admin 异步任务
                    webserver::star_webserver().await;
                    // 
                } => {
                    // println!("Received connection, shutting down...");
                }
            }

            info!( "Received SIGINT/SIGTERM, Web Admin Server shutting down...");
        })
    };
    webserver_handle
}

/// 获取 Instant Message Server 异步任务句柄
fn get_imserver_handle() -> tokio::task::JoinHandle<()> {
    let imserver_handle = {
        let mut sig_int = signal::unix::signal(signal::unix::SignalKind::interrupt()).unwrap();
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
                    info!("Instant Message Server starting...");
                    // 启动 Instant Message 异步任务
                    imserver::start_instant_message_server().await.expect("imserver error!");
                    // 
                } => {
                    // println!("Received connection, shutting down...");
                }
            }

            info!("Received SIGINT/SIGTERM, Instant Message Server shutting down...");
        })
    };
    imserver_handle
}

/// 获取 Message Queue Server 异步任务句柄
fn get_mqserver_handle() -> tokio::task::JoinHandle<()> {
    let mqserver_handle = {
        let mut sig_int = signal::unix::signal(signal::unix::SignalKind::interrupt()).unwrap();
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
                    // 启动 Message Queue 异步任务
                    info!("Message Queue Server starting...");
                    mqserver::start_message_event_queue_server().await.expect("mqserver error!");
                    // 
                } => {
                    info!("Received connection, shutting down...");
                }
            }

            info!( "Received SIGINT/SIGTERM, Message Queue Server shutting down...");
        })
    };
    mqserver_handle
}


// // 版权归亚马逊公司及其关联公司所有。保留所有权利。
// // SPDX-License-Identifier: Apache-2.0

// use btcmnetwork::{ imserver, mqserver, wdserver };
// use btcmweb::webserver;

// use std::error::Error;
// // use std::sync::atomic::{AtomicBool, Ordering};
// use std::sync::mpsc::channel;
// use ctrlc;
// use btcmtools::{ LOGGER, BitcommOpt };
// use slog::info;
// use tokio::signal;

// // use structopt::StructOpt;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let version: &'static str = env!("CARGO_PKG_VERSION");
//     info!(LOGGER, "bitcomm version = {}", version);
//     return start_server().await;
//     // 解析命令行参数
//     // let opt = BitcommOpt::from_args();
//     //
//     // switch_command(opt).await
// }

// fn _init_citric_system() {
//     let (tx, rx) = channel();

//     ctrlc
//         ::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
//         .expect("Error setting Ctrl-C handler");

//     tokio::spawn(async move {
//         println!("Waiting for Ctrl-C...");
//         rx.recv().expect("Could not receive from channel.");
//         println!("Got it! Exiting...");
//     });
// }
// #[allow(dead_code)]
// async fn switch_command(cmdopt: BitcommOpt) -> Result<(), Box<dyn Error>> {
//     match cmdopt {
//         BitcommOpt::StartServer => {
//             return start_server().await;
//         }
//         BitcommOpt::StopServer => {
//             stop_server();
//         }
//     }
//     Ok(())
// }

// fn stop_server() {
//     let pid = btcmtools::pid::read_pid();
//     if pid != -1 {
//         btcmtools::pid::kill_pid(pid);
//     } else {
//         info!(LOGGER, "Service not started");
//     }
//     btcmtools::pid::dele_pid();
// }

// async fn start_server() -> Result<(), Box<dyn Error>> {
//     // 写入PID
//     btcmtools::pid::save_pid();
//     // 输出日志
//     info!(LOGGER, "start server...");

//     // MQ Server
//     let mqserver_handle = get_mqserver_handle();

//     // IM Server
//     let imserver_handle = get_imserver_handle();

//     // Web Server
//     let webserver_handle = get_webserver_handle();

//     // WD Server
//     let wdserver_handle = get_wdserver_handle();

//     // 等待四个服务执行完毕
//     tokio::try_join!(mqserver_handle, imserver_handle, webserver_handle, wdserver_handle)?;
//     Ok(())
// }

// fn get_wdserver_handle() -> tokio::task::JoinHandle<()> {
//     let wdserver_handle = {
//         // let cpm = cpm.clone();
//         let mut sig_int = signal::unix::signal(signal::unix::SignalKind::interrupt()).unwrap();
//         let mut sig_term = signal::unix::signal(signal::unix::SignalKind::terminate()).unwrap();
//         tokio::spawn(async move {
//             tokio::select! {
//                 _ = async {
//                     // 等待中断信号
//                     sig_int.recv().await;
//                 } => {
//                     // info!(LOGGER,"Received SIGINT, watch dog server shutting down...");
//                 }
//                 _ = async {
//                     // 等待终止信号
//                     sig_term.recv().await;
//                 } => {
//                     // println!("Received SIGTERM, shutting down...");
//                 }
//                 _ = async {
//                     info!(LOGGER,"Watch Dog Server starting...");
//                     // 启动Watch Dog异步任务
//                     wdserver::start_watch_dog_server().await.expect("wdserver error!");
//                     // 
//                 } => {
//                     // println!("Received connection, shutting down...");
//                 }
//             }
//             info!(LOGGER, "Received SIGINT/SIGTERM, Watch Dog server shutting down...");
//         })
//     };
//     wdserver_handle
// }

// fn get_webserver_handle() -> tokio::task::JoinHandle<()> {
//     let webserver_handle = {
//         // let cpm = cpm.clone();
//         let mut sig_int = signal::unix::signal(signal::unix::SignalKind::interrupt()).unwrap();
//         let mut sig_term = signal::unix::signal(signal::unix::SignalKind::terminate()).unwrap();
//         tokio::spawn(async move {
//             tokio::select! {
//                 _ = async {
//                     // 等待中断信号
//                     sig_int.recv().await;
//                 } => {
//                     // println!("Received SIGINT, shutting down...");
//                 }
//                 _ = async {
//                     // 等待终止信号
//                     sig_term.recv().await;
//                 } => {
//                     // println!("Received SIGTERM, shutting down...");
//                 }
//                 _ = async {
//                     info!(LOGGER,"Web Admin Server starting...");
//                     // 启动Watch Dog异步任务
//                     webserver::star_webserver().await;
//                     // 
//                 } => {
//                     // println!("Received connection, shutting down...");
//                 }
//             }
//             info!(LOGGER, "Received SIGINT/SIGTERM, Web Admin Server shutting down...");
//         })
//     };
//     webserver_handle
// }

// fn get_imserver_handle() -> tokio::task::JoinHandle<()> {
//     let imserver_handle = {
//         // let cpm = cpm.clone();
//         // let meqsend = meqsend.clone();
//         let mut sig_int = signal::unix::signal(signal::unix::SignalKind::interrupt()).unwrap();
//         let mut sig_term = signal::unix::signal(signal::unix::SignalKind::terminate()).unwrap();
//         tokio::spawn(async move {
//             tokio::select! {
//                 _ = async {
//                     // 等待中断信号
//                     sig_int.recv().await;
//                 } => {
//                     // println!("Received SIGINT, shutting down...");
//                 }
//                 _ = async {
//                     // 等待终止信号
//                     sig_term.recv().await;
//                 } => {
//                     // println!("Received SIGTERM, shutting down...");
//                 }
//                 _ = async {
//                     info!(LOGGER,"Intant Message Server starting...");
//                     // 启动Watch Dog异步任务
//                     imserver::start_instant_message_server().await.expect("imserver error!");
//                     // 
//                 } => {
//                     // println!("Received connection, shutting down...");
//                 }
//             }
//             info!(LOGGER, "Received SIGINT/SIGTERM, Intant Message Server shutting down...");
//         })
//     };
//     imserver_handle
// }

// fn get_mqserver_handle() -> tokio::task::JoinHandle<()> {
//     let mqserver_handle = {
//         // let cpm = cpm.clone();
//         // let meqrece = meqrece.clone();
//         let mut sig_int = signal::unix::signal(signal::unix::SignalKind::interrupt()).unwrap();
//         let mut sig_term = signal::unix::signal(signal::unix::SignalKind::terminate()).unwrap();
//         tokio::spawn(async move {
//             tokio::select! {
//                 _ = async {
//                     // 等待中断信号
//                     sig_int.recv().await;
//                 } => {
//                     // println!("Received SIGINT, shutting down...");
//                 }
//                 _ = async {
//                     // 等待终止信号
//                     sig_term.recv().await;
//                 } => {
//                     // println!("Received SIGTERM, shutting down...");
//                 }
//                 _ = async {
//                     // 启动Watch Dog异步任务
//                     info!(LOGGER,"Message Queue Server starting...");
//                     mqserver::start_message_event_queue_server().await.expect("mqserver error!");
//                     // 
//                 } => {
//                     println!("Received connection, shutting down...");
//                 }
//             }
//             info!(LOGGER, "Received SIGINT/SIGTERM, Message Queue Server shutting down...");
//         })
//     };
//     mqserver_handle
// }
