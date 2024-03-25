mod config;
mod db;
mod todo_manager;

pub mod todo {
    tonic::include_proto!("todo");
    tonic::include_proto!("list");
    tonic::include_proto!("user");
}

use config::Config;
use todo::to_do_manager_server::ToDoManagerServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_config = Config::default();
    let todo_manager = todo_manager::BackendToDoManager::default();
    println!("ToDoServer listening on {}", server_config.server.addr);

    Server::builder()
        .add_service(ToDoManagerServer::new(todo_manager))
        .serve(server_config.server.addr)
        .await?;
    Ok(())
}
