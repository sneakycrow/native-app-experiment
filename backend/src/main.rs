use std::net::SocketAddr;

mod db;
mod manager;

pub mod todo {
    tonic::include_proto!("todo");
}

use todo::to_do_manager_server::ToDoManagerServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "[::1]:50051".parse().unwrap();
    let todo_manager = manager::BackendToDoManager::default();
    println!("ToDoServer listening on {}", addr);

    Server::builder()
        .add_service(ToDoManagerServer::new(todo_manager))
        .serve(addr)
        .await?;
    Ok(())
}
