use ::helloworld_grpc::*;
use grpc::ClientStubExt;

fn main() {
    let client = GreeterClient::new_plain("::1", 9999, Default::default()).unwrap();
    let mut req = HelloRequest::new();
    req.set_name(String::from("world"));
    let resp = client.say_hello(grpc::RequestOptions::new(), req);
    println!("{:?}", resp.wait());
}
