mod multiplex;
mod rust_grpc;

use std::io;

// 用于http 请求处理
use axum::{http::StatusCode, response::IntoResponse, Json};
use axum::{routing::{get, post},Router};
use multiplex::MultiplexService;

// grpc gen code import
use rust_grpc::hello::greeter_service_server::{GreeterService, GreeterServiceServer};
use rust_grpc::hello::{HelloReply, HelloReq};

use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};



// 用于序列化处理
use serde::{Deserialize, Serialize};

// trace
// use tracing::{error, info, Level};
use tracing::{info};
use tracing_subscriber::{fmt, EnvFilter};

// 这个file descriptor文件是build.rs中定义的descriptor_path路径
// 读取proto file descriptor bin二进制文件
pub(crate) const PROTO_FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("rust_grpc/rpc_descriptor.bin");

/// 实现hello.proto 接口服务
#[derive(Debug, Default)]
pub struct GreeterImpl {}

#[tonic::async_trait]
impl GreeterService for GreeterImpl {
    // 实现async_hello方法
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

/// 采用 tokio 运行时来跑grpc server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize tracing
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        // .with_max_level(Level::WARN)
        .with_file(true)
        .with_line_number(true)
        .with_writer(io::stdout) // 写入到标准输出
        // sets this to be the default, global collector for this application.
        .init();

    // build rest router
    let rest = Router::new().
        route("/", get(web_root))
        .route("/v1/greeter/say_hello", post(say_hello));

    // service address
    let address : SocketAddr = "0.0.0.0:3000".parse()?;
    println!("grpc server run on:{}", address);

    // grpc reflection服务
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(PROTO_FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let greeter = GreeterImpl::default();
    let grpc_service = Server::builder()
        .add_service(reflection_service)
        .add_service(GreeterServiceServer::new(greeter))
        .into_service();

    // combine them into one service
    let service = MultiplexService::new(rest, grpc_service);

    info!("listening on {}", address);

    // run multiplex service
    axum::Server::bind(&address)
        .serve(tower::make::Shared::new(service))
        .await?;

    Ok(())
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
