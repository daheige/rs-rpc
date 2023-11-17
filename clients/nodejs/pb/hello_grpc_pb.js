// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var hello_pb = require('./hello_pb.js');
// var google_api_annotations_pb = require('./google/api/annotations_pb.js');

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


// GreeterService 定义服务
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
};

exports.GreeterServiceClient = grpc.makeGenericClientConstructor(GreeterServiceService);
