mod hybrid_service;

use rust_grpc::hello::greeter_service_server::{GreeterService, GreeterServiceServer};
use rust_grpc::hello::{HelloReply, HelloReq};

use tonic::{transport::Server, Request, Response, Status};

/// 定义grpc代码生成的包名
mod rust_grpc;

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
    let address = "127.0.0.1:50051".parse()?;
    println!("grpc server run on:{}", address);

    let axum_make_service = axum::Router::new()
        .route("/", axum::routing::get(|| async { "Hello world!" }))
        .into_make_service();

    // grpc reflection服务
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(PROTO_FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    // grpc service
    let greeter = GreeterImpl::default();
    let grpc_service = Server::builder()
        .add_service(reflection_service)
        .add_service(GreeterServiceServer::new(greeter))
        .into_service();

    // hybrid server
    let hybrid_make_service = hybrid_service::hybrid(axum_make_service,grpc_service);
    let server = hyper::Server::bind(&address).serve(hybrid_make_service);
    // if let Err(err) = server.await{
    //     println!("server error: {}", err);
    // }

    server.await?;

    Ok(())
}
