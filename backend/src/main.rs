mod config;
mod db;
mod manager;

pub mod todo {
    tonic::include_proto!("todo");
}

use config::Config;
use todo::to_do_manager_server::ToDoManagerServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_config = Config::default();
    let todo_manager = manager::BackendToDoManager::default();
    println!("ToDoServer listening on {}", server_config.server.addr);

    Server::builder()
        .add_service(ToDoManagerServer::new(todo_manager))
        .serve(server_config.server.addr)
        .await?;
    Ok(())
}
