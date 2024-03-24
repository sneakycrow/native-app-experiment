use std::net::SocketAddr;

use tonic::{transport::Server, Request, Response, Status};

use todo::to_do_manager_server::{ToDoManager, ToDoManagerServer};
use todo::{CreateToDoRequest, ToDoResponse, FindToDoRequest, DeleteToDoRequest, UpdateToDoRequest};
pub mod todo {
    tonic::include_proto!("todo");
}

#[derive(Default)]
pub struct BackendToDoManager {}

#[tonic::async_trait]
impl ToDoManager for BackendToDoManager {
    async fn create(&self, request: Request<CreateToDoRequest>) -> Result<Response<ToDoResponse>, Status> {
        todo!("Implement")
    }
    async fn find(&self, request: Request<FindToDoRequest>) -> Result<Response<ToDoResponse>, Status> {
        todo!("Implement")
    }
    async fn delete(&self, request: Request<DeleteToDoRequest>) -> Result<Response<ToDoResponse>, Status> {
        todo!("Implement")
    }
    async fn update(&self, request: Request<UpdateToDoRequest>) -> Result<Response<ToDoResponse>, Status> {
        todo!("Implement")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "[::1]:50051".parse().unwrap();
    let todo_manager = BackendToDoManager::default();
    println!("ToDoServer listening on {}", addr);

    Server::builder()
        .add_service(ToDoManagerServer::new(todo_manager))
        .serve(addr)
        .await?;
    Ok(())
}