// 如果想使用grpc-go http gateway请使用main.go中的方式
// grpc-go http gateway实现参考：https://github.com/grpc-ecosystem/grpc-gateway
// 也可以使用rust axum http处理
mod rust_grpc;
use rust_grpc::hello::greeter_service_client::GreeterServiceClient;
use rust_grpc::hello::HelloReq;

// 用于http 请求处理
use axum::routing::{get, post};
use axum::{http::StatusCode, response::IntoResponse, Json,Router};
use tonic::Request;

// 用于序列化处理
use serde::{Deserialize, Serialize};

// 用于http 启动
use std::net::SocketAddr;
use std::process;
use std::time::Duration;
use tokio::signal;

#[derive(Deserialize, Serialize, Debug)]
pub struct Reply<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

const GRPC_ADDRESS : &str = "http://127.0.0.1:8081";

// 将请求反序列化到HelloReq，然后调用grpc service
async fn say_hello(Json(payload): Json<HelloReq>) -> impl IntoResponse {
    let req = Request::new(payload);
    let client = GreeterServiceClient::connect(GRPC_ADDRESS).await;
    if let Err(err) = client {
        return (
            StatusCode::OK,
            Json(Reply {
                code: 500,
                message: format!("request err:{}", err),
                data: None,
            }),
        );
    }

    let mut client = client.unwrap();
    println!("client:{:?}", client);

    let response = client.say_hello(req).await;
    println!("res:{:?}",response);
    match response {
        Ok(res) => {
            let res = res.into_inner();

            (
                StatusCode::OK,
                Json(Reply {
                    code: 0,
                    message: "ok".to_string(),
                    data: Some(res),
                }),
            )
        }
        Err(err) => (
            StatusCode::OK,
            Json(Reply {
                code: 500,
                message: format!("request err:{}", err),
                data: None,
            }),
        ),
    }
}

// 运行这个main.rs之前，请先启动src/main.rs启动rust grpc service
#[tokio::main]
async fn main() {
    println!("rs-rpc http gateway");
    let router = Router::new()
        .route("/", get(|| async { "Hello world!" }))
        .route("/v1/greeter/say_hello", post(say_hello));

    println!("current process pid:{}", process::id());

    let address: SocketAddr = format!("127.0.0.1:{}", 8090)
        .parse()
        .unwrap();
    println!("app run on:{}", address.to_string());

    // run app
    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .with_graceful_shutdown(graceful_shutdown())
        .await
        .unwrap();
}

// graceful shutdown
async fn graceful_shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install ctrl+c handler");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c =>{
            println!("received ctrl_c signal,server will exist...");
            tokio::time::sleep(Duration::from_secs(5)).await;
        },
        _ = terminate => {
            println!("received terminate signal,server will exist...");
            tokio::time::sleep(Duration::from_secs(5)).await;
        },
    }

    println!("signal received,starting graceful shutdown");
}
