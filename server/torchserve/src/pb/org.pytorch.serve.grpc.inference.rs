#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictionsRequest {
    /// Name of model.
    ///
    ///required
    #[prost(string, tag="1")]
    pub model_name: ::prost::alloc::string::String,
    /// Version of model to run prediction on.
    ///
    ///optional
    #[prost(string, tag="2")]
    pub model_version: ::prost::alloc::string::String,
    /// input data for model prediction
    ///
    ///required
    #[prost(map="string, bytes", tag="3")]
    pub input: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictionResponse {
    /// TorchServe health
    #[prost(bytes="vec", tag="1")]
    pub prediction: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TorchServeHealthResponse {
    /// TorchServe health
    #[prost(string, tag="1")]
    pub health: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod inference_ap_is_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct InferenceApIsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InferenceApIsServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> InferenceApIsServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InferenceApIsServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            InferenceApIsServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn ping(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> Result<tonic::Response<super::TorchServeHealthResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/org.pytorch.serve.grpc.inference.InferenceAPIsService/Ping",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Predictions entry point to get inference using default model version.
        pub async fn predictions(
            &mut self,
            request: impl tonic::IntoRequest<super::PredictionsRequest>,
        ) -> Result<tonic::Response<super::PredictionResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/org.pytorch.serve.grpc.inference.InferenceAPIsService/Predictions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
