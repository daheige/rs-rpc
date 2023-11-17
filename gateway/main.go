package main

import (
	"context"
	"flag"
	"log"
	"net/http"

	"github.com/grpc-ecosystem/grpc-gateway/v2/runtime"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"

	gw "github.com/daheige/rs-rpc/clients/go/pb" // Update this package path
)

var (
	// command-line options:
	// gRPC server endpoint
	grpcServerEndpoint string

	// gRPC http gateway address
	grpcGatewayAddress string
)

func init() {
	flag.StringVar(&grpcServerEndpoint, "grpc-server-endpoint", "localhost:8081", "gRPC server endpoint")
	flag.StringVar(&grpcGatewayAddress, "grpc-http-gateway", "localhost:8090", "gRPC http gateway address")
	flag.Parse()
}

func run() error {
	ctx := context.Background()
	ctx, cancel := context.WithCancel(ctx)
	defer cancel()

	// Register gRPC server endpoint
	// Note: Make sure the gRPC server is running properly and accessible
	mux := runtime.NewServeMux()
	opts := []grpc.DialOption{grpc.WithTransportCredentials(insecure.NewCredentials())}
	err := gw.RegisterGreeterServiceHandlerFromEndpoint(ctx, mux, grpcServerEndpoint, opts)
	if err != nil {
		return err
	}

	log.Println("grpc server endpoint run on: ", grpcServerEndpoint)
	log.Println("http gateway run on: ", grpcGatewayAddress)

	// Start HTTP server (and proxy calls to gRPC server endpoint)
	return http.ListenAndServe(grpcGatewayAddress, mux)
}

func main() {
	// run http gateway
	if err := run(); err != nil {
		log.Fatalln("http gateway run error: ", err)
	}
}
