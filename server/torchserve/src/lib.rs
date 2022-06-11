//! TorchServe gRPC rust client

mod pb;

use std::collections::HashMap;
use pb::org::pytorch::serve::grpc::inference::{inference_ap_is_service_client::InferenceApIsServiceClient, *};
use tonic::transport::Channel;
use tracing::Instrument;
use tracing::*;

#[derive(Debug, Clone)]
pub struct TorchServeClient{
    stub: InferenceApIsServiceClient<Channel>
}

impl TorchServeClient{
    pub fn new(channel: Channel) -> Self {
        Self {stub: InferenceApIsServiceClient::new(channel)}
    }

    pub async fn connect(dst: String) -> Self {
        let channel = tonic::transport::Endpoint::from_shared(dst)
            .expect("TorchServe URL is not valid")
            .connect()
            .await
            .expect("Unable to connect to TorchServe");
        Self::new(channel)
    }

    #[instrument(skip(self))]
    pub async fn ping(&mut self) -> Result<String, tonic::Status> {
        let response = self.stub.ping(()).instrument(info_span!("ping")).await?;
        Ok(response.into_inner().health)
    }

    #[instrument(skip(self, inputs))]
    pub async fn predictions(&mut self, inputs: HashMap<String, Vec<u8>>, model_name: &str, model_version: &str) -> Result<Vec<u8>, tonic::Status> {
        let request = PredictionsRequest{
            model_name: model_name.to_string(),
            model_version: model_version.to_string(),
            input: inputs
        };

        let response = self.stub.predictions(request).instrument(info_span!("predictions")).await?;
        Ok(response.into_inner().prediction)
    }
}