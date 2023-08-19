use chat::chat_server::{Chat, ChatServer};
use chat::{ConnectServerRequest, Message, SendMessageRequest};
use futures::Stream;
use greeting::greeter_server::{Greeter, GreeterServer};
use greeting::{GreetingMessage, Person};
use std::collections::HashMap;
use std::iter::Empty;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;

pub mod greeting {
    // greeting.protoから生成したRustコードを展開するマクロ
    tonic::include_proto!("greeting");
}

pub mod chat {
    tonic::include_proto!("chat");
}

#[derive(Default)]
struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<Person>,
    ) -> Result<Response<GreetingMessage>, Status> {
        // gRPCリクエストから入力値を参照する
        let name = request.into_inner().name;
        println!("Creating a greeting message for {:?}", name);
        // レスポンスの内容を作成する
        let greeting_message = GreetingMessage {
            text: format!("Hello {}!", name),
        };
        // gRPCレスポンスを作成する
        let response = Response::new(greeting_message);
        // gRPCレスポンスを返す
        Ok(response)
    }
}

#[derive(Debug)]
struct Shared {
    senders: HashMap<String, mpsc::Sender<Message>>,
}

impl Shared {
    fn new() -> Self {
        Shared {
            senders: HashMap::new(),
        }
    }

    async fn broadcast(&self, message: Message) {
        for (name, tx) in &self.senders {
            match tx.send(message.clone()).await {
                Ok(_) => {}
                Err(_) => {
                    println!("[Broadcast] SendError: to {}, {:?}", name, message)
                }
            }
        }
    }
}

// proto related
#[derive(Debug)]
struct ChatService {
    shared: Arc<RwLock<Shared>>,
}

impl ChatService {
    fn new(shared: Arc<RwLock<Shared>>) -> Self {
        ChatService { shared }
    }
}

#[tonic::async_trait]
impl Chat for ChatService {
    type ConnectServerStream =
        Pin<Box<dyn Stream<Item = Result<Message, Status>> + Send + Sync + 'static>>;

    async fn connect_server(
        &self,
        request: Request<ConnectServerRequest>,
    ) -> Result<Response<Self::ConnectServerStream>, Status> {
        let name = request.into_inner().user_name;
        let (stream_tx, stream_rx) = mpsc::channel(1); // Fn usage

        // When connecting, create related sender and reciever
        let (tx, mut rx) = mpsc::channel(1);
        {
            self.shared.write().await.senders.insert(name.clone(), tx);
        }

        let shared_clone = self.shared.clone();
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                match stream_tx.send(Ok(msg)).await {
                    Ok(_) => {}
                    Err(_) => {
                        // If sending failed, then remove the user from shared data
                        println!("[Remote] stream tx sending error. Remote {}", &name);
                        shared_clone.write().await.senders.remove(&name);
                    }
                }
            }
        });

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(stream_rx),
        )))
    }

    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<Message>, Status> {
        let req_data = request.into_inner();
        let message = req_data.message.unwrap();
        let user_name = message.user_name;
        let content = message.content;
        let msg: Message = Message { user_name, content };
        self.shared.read().await.broadcast(msg.clone()).await;

        Ok(Response::new(msg))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    println!("SendMsg Server listening on: {}", addr);

    let shared = Arc::new(RwLock::new(Shared::new()));
    let chat_service = ChatService::new(shared.clone());

    let allow_cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any);

    Server::builder()
        .accept_http1(true) // gRPC-webに対応するために必要
        .layer(allow_cors) // CORSに対応するために必要
        .layer(GrpcWebLayer::new()) // gRPC-webに対応するために必要
        .add_service(ChatServer::new(chat_service))
        .serve(addr)
        .await?;

    Ok(())
}
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let addr = "127.0.0.1:50051".parse().unwrap();
//     let greeter = MyGreeter::default();
//     let allow_cors = CorsLayer::new()
//         .allow_origin(tower_http::cors::Any)
//         .allow_headers(tower_http::cors::Any)
//         .allow_methods(tower_http::cors::Any);
//     println!("GreeterServer listening on {}", addr);
//     Server::builder()
//         .accept_http1(true) // gRPC-webに対応するために必要
//         .layer(allow_cors) // CORSに対応するために必要
//         .layer(GrpcWebLayer::new()) // gRPC-webに対応するために必要
//         .add_service(GreeterServer::new(greeter))
//         .serve(addr)
//         .await?;
//     Ok(())
// }
