<?php
// GENERATED CODE -- DO NOT EDIT!

namespace App\Grpc\Hello;

/**
 * 如果需要http gateway的话，请打开这个注释
 * import "google/api/annotations.proto";
 *
 */
class GreeterServiceClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * @param \App\Grpc\Hello\HelloReq $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function SayHello(\App\Grpc\Hello\HelloReq $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/Hello.GreeterService/SayHello',
        $argument,
        ['\App\Grpc\Hello\HelloReply', 'decode'],
        $metadata, $options);
    }

}
