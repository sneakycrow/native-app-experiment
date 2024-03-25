use crate::todo::to_do_manager_server::ToDoManager;
use crate::todo::{
    CreateToDoRequest, DeleteToDoRequest, FindToDoRequest, ToDoResponse, UpdateToDoRequest,
};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct BackendToDoManager {}

#[tonic::async_trait]
impl ToDoManager for BackendToDoManager {
    async fn create(
        &self,
        request: Request<CreateToDoRequest>,
    ) -> Result<Response<ToDoResponse>, Status> {
        todo!("Implement")
    }
    async fn find(
        &self,
        request: Request<FindToDoRequest>,
    ) -> Result<Response<ToDoResponse>, Status> {
        todo!("Implement")
    }
    async fn delete(
        &self,
        request: Request<DeleteToDoRequest>,
    ) -> Result<Response<ToDoResponse>, Status> {
        todo!("Implement")
    }
    async fn update(
        &self,
        request: Request<UpdateToDoRequest>,
    ) -> Result<Response<ToDoResponse>, Status> {
        todo!("Implement")
    }
}
