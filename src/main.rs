use serde::{Deserialize, Serialize};
use zlink::{
    self, Call, Connection, ReplyError, Server, Service, connection::Socket, service::MethodReply,
    unix, varlink_service::Info,
};

const SOCKET_PATH: &str = "/tmp/hello.varlink";

#[tokio::main]
async fn main() {
    println!("starting varlink hello world server");

    run_server().await;
}

#[derive(Debug, ReplyError)]
#[zlink(interface = "rocks.dtz.HelloWorld")]
enum HelloWorldError {
    Error { message: String },
}

pub async fn run_server() {
    let _ = tokio::fs::remove_file(SOCKET_PATH).await;
    let listener = unix::bind(SOCKET_PATH).unwrap();
    let service = HelloWorld {};
    let server = Server::new(listener, service);

    match server.run().await {
        Ok(_) => println!("server done."),
        Err(e) => println!("server error: {:?}", e),
    }
}

struct HelloWorld {}

// Implement the Service trait
impl Service for HelloWorld {
    type MethodCall<'de> = HelloWorldMethod;
    type ReplyParams<'ser> = HelloWorldReply;
    type ReplyStreamParams = ();
    type ReplyStream = futures_util::stream::Empty<zlink::Reply<()>>;
    type ReplyError<'ser> = HelloWorldError;

    async fn handle<'ser, 'de: 'ser, Sock: Socket>(
        &'ser mut self,
        call: Call<Self::MethodCall<'de>>,
        _conn: &mut Connection<Sock>,
    ) -> MethodReply<Self::ReplyParams<'ser>, Self::ReplyStream, Self::ReplyError<'ser>> {
        println!("handling call: {:?}", call.method());
        match call.method() {
            HelloWorldMethod::Hello => {
                MethodReply::Single(Some(HelloWorldReply::Hello(HelloResponse {
                    message: "Hello, World!".to_string(),
                })))
            }
            HelloWorldMethod::NamedHello { parameters } => {
                MethodReply::Single(Some(HelloWorldReply::Hello(HelloResponse {
                    message: format!("Hello, {}!", parameters.name),
                })))
            }
            HelloWorldMethod::VarlinkGetInfo => {
                MethodReply::Single(Some(HelloWorldReply::VarlinkInfo(Info::<'static> {
                    vendor: "DownToZero",
                    product: "hello-world",
                    url: "https://github.com/DownToZero-Cloud/varlink-helloworld",
                    interfaces: vec!["rocks.dtz.HelloWorld", "org.varlink.service"],
                    version: "1.0.0",
                })))
            }
        }
    }
}

// Method calls the service handles
#[derive(Debug, Deserialize)]
#[serde(tag = "method")]
enum HelloWorldMethod {
    #[serde(rename = "rocks.dtz.HelloWorld.Hello")]
    Hello,
    #[serde(rename = "rocks.dtz.HelloWorld.NamedHello")]
    NamedHello {
        #[serde(default)]
        parameters: NamedHelloParameters,
    },
    #[serde(rename = "org.varlink.service.GetInfo")]
    VarlinkGetInfo,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NamedHelloParameters {
    name: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum HelloWorldReply {
    Hello(HelloResponse),
    VarlinkInfo(Info<'static>),
}

#[derive(Debug, Serialize)]
pub struct HelloResponse {
    message: String,
}
