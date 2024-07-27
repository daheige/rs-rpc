let services = require('./pb/hello_grpc_pb.js');
let messages = require('./pb/hello_pb.js');
let grpc = require('@grpc/grpc-js');

let request = new messages.HelloReq();
request.setName('heige');

let client = new services.GreeterServiceClient(
    'localhost:50051',
    // 'localhost:8090', // nginx grpc pass port
    grpc.credentials.createInsecure()
);

client.sayHello(request, function(err, data) {
    if (err) {
        console.error(err);
        return;
    }

    console.log(data);
    console.log("message: ",data.getMessage());
    console.log("name: ",data.getName());
});
