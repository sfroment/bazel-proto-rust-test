use std::thread;

use ::grpc::*;
use ::helloworld_grpc::*;
use ::tls_api_stub::*;

struct GreeterImpl;

impl Greeter for GreeterImpl {
    fn say_hello(
        &self,
        _m: grpc::RequestOptions,
        req: HelloRequest,
    ) -> grpc::SingleResponse<HelloReply> {
        let mut r = HelloReply::new();
        let name = if req.get_name().is_empty() {
            "world"
        } else {
            req.get_name()
        };
        println!("greeting request from {name}");
        r.set_message(format!("Hello {name}"));
        grpc::SingleResponse::completed(r)
    }
}

fn main() {
    let mut server = grpc::ServerBuilder::<tls_api_stub::TlsAcceptor>::new();
    server.http.set_port(9999);
    server.add_service(GreeterServer::new_service_def(GreeterImpl));
    server.http.set_cpu_pool_threads(4);
    let server = server.build().expect("server");
    let port = server.local_addr().port().unwrap();
    println!("greeter server started on port {port}");
    loop {
        thread::park();
    }
}
