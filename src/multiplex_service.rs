use autometrics::autometrics;
use axum::{
    extract::Request as AxumRequest,
    http::header::CONTENT_TYPE,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use infras::metrics::prometheus_init;
use rust_grpc::hello::greeter_service_server::{GreeterService, GreeterServiceServer};
use rust_grpc::hello::{HelloReply, HelloReq};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;
use tonic::{transport::Server, Request, Response, Status};
use tower::{make::Shared, steer::Steer};

mod infras;
mod rust_grpc;

// 这个file descriptor文件是build.rs中定义的descriptor_path路径
// 读取proto file descriptor bin二进制文件
pub(crate) const PROTO_FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("rust_grpc/rpc_descriptor.bin");

/// 实现hello.proto 接口服务
#[derive(Debug, Default)]
pub struct GreeterImpl {}

#[async_trait::async_trait]
impl GreeterService for GreeterImpl {
    // 实现async_hello方法
    #[autometrics]
    async fn say_hello(&self, request: Request<HelloReq>) -> Result<Response<HelloReply>, Status> {
        // 获取request pb message
        let req = &request.into_inner();
        println!("got request.id:{}", req.id);
        println!("got request.name:{}", req.name);
        let reply = HelloReply {
            message: format!("hello,{}", req.name),
            name: format!("{}", req.name).into(),
        };

        Ok(Response::new(reply))
    }
}

async fn web_root() -> &'static str {
    "Hello, World!"
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Reply<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

// 将请求反序列化到HelloReq，然后调用grpc service
#[autometrics]
async fn say_hello(Json(payload): Json<HelloReq>) -> impl IntoResponse {
    let req = Request::new(payload);
    let greeter = GreeterImpl::default();
    let response = greeter.say_hello(req).await;
    match response {
        Ok(res) => {
            let reply = res.into_inner();
            (
                StatusCode::OK,
                Json(Reply {
                    code: 0,
                    message: "ok".to_string(),
                    data: Some(reply),
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

/// 采用 tokio 运行时来跑grpc server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address: SocketAddr = "0.0.0.0:8081".parse()?;
    println!("grpc server and http server run on:{}", address);

    // grpc reflection服务
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(PROTO_FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let greeter = GreeterImpl::default();
    let grpc_server = Server::builder()
        .add_service(reflection_service)
        .add_service(GreeterServiceServer::new(greeter))
        .into_router();

    // build the rest service
    let rest_server = Router::new()
        .route("/", get(web_root))
        .route("/v1/greeter/say_hello", post(say_hello));

    // combine them into one service
    let service = Steer::new(
        vec![rest_server, grpc_server],
        |req: &AxumRequest, _services: &[_]| {
            if req
                .headers()
                .get(CONTENT_TYPE)
                .map(|content_type| content_type.as_bytes())
                .filter(|content_type| content_type.starts_with(b"application/grpc"))
                .is_some()
            {
                // grpc service
                1
            } else {
                // http service
                0
            }
        },
    );

    // create http /metrics endpoint
    let metrics_server = prometheus_init(8091);
    let metrics_handler = tokio::spawn(metrics_server);
    let multiplex_handler = tokio::spawn(async move {
        // run multiplex service on one port
        let listener = TcpListener::bind(&address).await.unwrap();
        axum::serve(listener, Shared::new(service))
            .with_graceful_shutdown(infras::shutdown::graceful_shutdown(Duration::from_secs(3)))
            .await
            .expect("failed to start multiplex service");
    });

    // run async tasks by tokio try_join macro
    let _ = tokio::try_join!(metrics_handler, multiplex_handler);
    Ok(())
}
