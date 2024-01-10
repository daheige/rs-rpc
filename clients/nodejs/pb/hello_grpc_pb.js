// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var hello_pb = require('./hello_pb.js');

function serialize_Hello_HelloReply(arg) {
  if (!(arg instanceof hello_pb.HelloReply)) {
    throw new Error('Expected argument of type Hello.HelloReply');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_Hello_HelloReply(buffer_arg) {
  return hello_pb.HelloReply.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_Hello_HelloReq(arg) {
  if (!(arg instanceof hello_pb.HelloReq)) {
    throw new Error('Expected argument of type Hello.HelloReq');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_Hello_HelloReq(buffer_arg) {
  return hello_pb.HelloReq.deserializeBinary(new Uint8Array(buffer_arg));
}


// 如果需要http gateway的话，请打开这个注释
// import "google/api/annotations.proto";
//
var GreeterServiceService = exports.GreeterServiceService = {
  sayHello: {
    path: '/Hello.GreeterService/SayHello',
    requestStream: false,
    responseStream: false,
    requestType: hello_pb.HelloReq,
    responseType: hello_pb.HelloReply,
    requestSerialize: serialize_Hello_HelloReq,
    requestDeserialize: deserialize_Hello_HelloReq,
    responseSerialize: serialize_Hello_HelloReply,
    responseDeserialize: deserialize_Hello_HelloReply,
  },
  // 如果需要http gateway的话，请打开这个注释
//        option (google.api.http) = {
//            // get: "/v1/greeter/say_hello/{name}"
//            post: "/v1/greeter/say_hello"
//            body: "*"
//        };
};

exports.GreeterServiceClient = grpc.makeGenericClientConstructor(GreeterServiceService);
