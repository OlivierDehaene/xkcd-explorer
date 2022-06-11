use crate::object::{Comic, ComicRequest};
use crate::record::Record;
use crate::{xkcd_explorer_server, Database, Empty, ExistsResponse};
use std::collections::HashMap;
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};
use torchserve::TorchServeClient;
use tracing::instrument;

#[derive(Debug)]
pub struct XkcdExplorerService {
    client: TorchServeClient,
    model_name: String,
    model_version: String,
    data: RwLock<Database>,
}

impl XkcdExplorerService {
    pub fn new(
        client: TorchServeClient,
        model_name: String,
        model_version: String,
        data: RwLock<Database>,
    ) -> Self {
        Self {
            client,
            model_name,
            model_version,
            data,
        }
    }
}

#[tonic::async_trait]
impl xkcd_explorer_server::XkcdExplorer for XkcdExplorerService {
    #[instrument(skip_all, fields(request, response))]
    async fn exists(
        &self,
        request: Request<ComicRequest>,
    ) -> Result<Response<ExistsResponse>, Status> {
        let request = request.record().into_inner();

        let database = &*self.data.read().await;
        if database.exists(&request.id) {
            Ok(Response::new(ExistsResponse { exists: true }).record())
        } else {
            Ok(Response::new(ExistsResponse { exists: false }).record())
        }
    }

    #[instrument(skip_all, fields(request, response))]
    async fn get(&self, request: Request<ComicRequest>) -> Result<Response<Comic>, Status> {
        let request = request.record().into_inner();

        let database = &*self.data.read().await;
        match database.get(&request.id) {
            None => Err(Status::not_found(format!("id {} not found", &request.id))).record(),
            Some(comic) => Ok(Response::new(comic.clone()).record()),
        }
    }

    #[instrument(skip_all, fields(request, response))]
    async fn search(
        &self,
        request: Request<crate::search::Request>,
    ) -> Result<Response<crate::search::Response>, Status> {
        let request = request.record().into_inner();

        let inputs: HashMap<String, Vec<u8>> =
            HashMap::from([("data".to_string(), request.prompt.into_bytes())]);
        let embeddings = self
            .client
            .clone()
            .predictions::<Vec<f32>>(inputs, &self.model_name, &self.model_version)
            .await?;

        let database = &mut *self.data.write().await;
        let results = database
            .search(embeddings, 10)
            .map_err(|err| Status::unavailable(err.to_string()))
            .record()?;
        Ok(Response::new(crate::search::Response { results }).record())
    }

    #[instrument(skip_all, fields(request))]
    async fn insert(&self, request: Request<Comic>) -> Result<Response<Empty>, Status> {
        let request = request.record().into_inner();

        let inputs: HashMap<String, Vec<u8>> =
            HashMap::from([("data".to_string(), request.text.clone().into_bytes())]);
        let embeddings = self
            .client
            .clone()
            .predictions::<Vec<f32>>(inputs, &self.model_name, &self.model_version)
            .await
            .record()?;

        let database = &mut *self.data.write().await;
        database
            .insert(request, embeddings)
            .map_err(|err| Status::unavailable(err.to_string()))
            .record()?;
        Ok(Response::new(Empty {}).record())
    }
}
