use rust_grpc::hello::greeter_service_client::GreeterServiceClient;
use rust_grpc::hello::HelloReq;

// tonic request
use tonic::Request;

mod rust_grpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = Request::new(HelloReq {
        id: 1,
        name: "daheige".into(),
    });

    let address = "http://127.0.0.1:50051";
    // let address = "http://127.0.0.1:8081";
    let mut client = GreeterServiceClient::connect(address).await?;
    println!("client:{:?}", client);

    let response = client.say_hello(request).await?;
    println!("res:{:?}", response);

    let res = response.into_inner();
    println!("name:{}", res.name);
    println!("message:{}", res.message);
    Ok(())
}
