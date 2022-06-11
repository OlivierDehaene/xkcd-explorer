use std::collections::HashMap;
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};
use crate::{Empty, ExistsResponse, xkcd_explorer_server};
use crate::object::{Comic, ComicRequest};
use crate::record::Record;
use tracing::instrument;

#[derive(Debug)]
pub struct XkcdExplorerService{
    comics: RwLock<HashMap<i32, Comic>>
}

impl XkcdExplorerService{
    pub fn new(comics: RwLock<HashMap<i32, Comic>>) -> Self {
        Self{comics}
    }
}


#[tonic::async_trait]
impl xkcd_explorer_server::XkcdExplorer for XkcdExplorerService{
    #[instrument(skip_all, fields(request, response))]
    async fn exists(&self, request: Request<ComicRequest>) -> Result<Response<ExistsResponse>, Status> {
        let request = request.record().into_inner();

        let database = &*self.comics.read().await;
        let response = if database.contains_key(&request.id) {
            Ok(Response::new(ExistsResponse { exists: true }))
        } else {
            Ok(Response::new(ExistsResponse { exists: false }))
        };
        response.record()
    }

    #[instrument(skip_all, fields(request, response))]
    async fn get(&self, request: Request<ComicRequest>) -> Result<Response<Comic>, Status> {
        let request = request.record().into_inner();

        let database = &*self.comics.read().await;
        let response = match database.get(&request.id) {
            None => Err(Status::not_found(format!("id {} not found", &request.id))),
            Some(comic) => Ok(Response::new(comic.clone()))
        };
        response.record()
    }

    #[instrument(skip_all, fields(request, response))]
    async fn search(&self, request: Request<crate::search::Request>) -> Result<Response<crate::search::Response>, Status> {
        todo!()
    }

    #[instrument(skip_all, fields(request))]
    async fn insert(&self, request: Request<Comic>) -> Result<Response<Empty>, Status> {
        let request = request.record().into_inner();

        let database = &mut *self.comics.write().await;
        database.insert(request.id, request);
        Ok(Response::new(Empty{})).record()
    }
}