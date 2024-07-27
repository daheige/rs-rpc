// 如果想使用grpc-go http gateway请使用main.go中的方式
// grpc-go http gateway实现参考：https://github.com/grpc-ecosystem/grpc-gateway
// 也可以使用rust axum http处理
mod rust_grpc;
use rust_grpc::hello::greeter_service_client::GreeterServiceClient;
use rust_grpc::hello::HelloReq;

// 用于http 请求处理
use axum::routing::{get, post};
use axum::{http::StatusCode, response::IntoResponse, Json, Router};
use tonic::Request;

// 用于序列化处理
use serde::{Deserialize, Serialize};

// 用于http 启动
use std::net::SocketAddr;
use std::process;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::signal;

#[derive(Deserialize, Serialize, Debug)]
pub struct Reply<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

const GRPC_ADDRESS: &str = "http://127.0.0.1:8081";

// 将请求反序列化到HelloReq，然后调用grpc service
async fn say_hello(Json(payload): Json<HelloReq>, state: Arc<AppState>) -> impl IntoResponse {
    let req = Request::new(payload);
    let response = state.grpc_client.clone().say_hello(req).await;
    println!("res:{:?}", response);
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

// 定义传递给axum handlers的app_state，这里是通过引用计数的方式共享变量
// Sharing state with handlers
struct AppState {
    grpc_client: GreeterServiceClient<tonic::transport::Channel>,
}

// 运行这个main.rs之前，请先启动src/main.rs启动rust grpc service
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("rs-rpc http gateway");
    // create grpc client
    let grpc_client = GreeterServiceClient::connect(GRPC_ADDRESS).await?;

    // 通过arc引用计数的方式传递state
    let app_state = Arc::new(AppState { grpc_client });

    // create http router
    let router = Router::new()
        .route("/", get(|| async { "Hello world!" }))
        .route(
            "/v1/greeter/say_hello",
            post({
                // 这里通过代码块和闭包的方式传递state
                // State can also be passed directly to handlers using closure captures
                let app_state = app_state.clone();
                move |body| say_hello(body, app_state)
            }),
        );

    println!("current process pid:{}", process::id());

    // run app
    let address: SocketAddr = "0.0.0.0:8090".parse().unwrap();
    println!("app run on:{}", address.to_string());

    // Create a `TcpListener` using tokio.
    let listener = TcpListener::bind(address).await.unwrap();

    // // run multiplex service with graceful shutdown
    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(graceful_shutdown())
        .await?;

    Ok(())
}

// graceful shutdown
async fn graceful_shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
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
