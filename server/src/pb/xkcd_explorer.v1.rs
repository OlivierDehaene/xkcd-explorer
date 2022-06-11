/// Represent a Exists response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExistsResponse {
    /// True if id exists, False otherwise
    #[prost(bool, tag="1")]
    pub exists: bool,
}
/// Search related messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Search {
}
/// Nested message and enum types in `Search`.
pub mod search {
    /// Represent a search request.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        /// Prompt to use for asymmetric semantic search
        #[prost(string, tag="1")]
        pub prompt: ::prost::alloc::string::String,
        /// Number of results to return.
        #[prost(uint32, tag="3")]
        pub topk: u32,
    }
    /// Represent a search response.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// Search results.
        #[prost(message, repeated, tag="1")]
        pub results: ::prost::alloc::vec::Vec<super::object::Similarity>,
    }
}
/// Common messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Object {
}
/// Nested message and enum types in `Object`.
pub mod object {
    /// Represent a Comic
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Comic {
        /// The id to insert.
        #[prost(uint64, tag="1")]
        pub id: u64,
        /// The comic title
        #[prost(string, tag="2")]
        pub title: ::prost::alloc::string::String,
        /// The comic xkcd.com url
        #[prost(string, tag="3")]
        pub url: ::prost::alloc::string::String,
        /// The comic image url
        #[prost(string, tag="4")]
        pub image_url: ::prost::alloc::string::String,
        /// The comic explainxkcd.com url
        #[prost(string, tag="5")]
        pub explained_url: ::prost::alloc::string::String,
        /// Text
        #[prost(string, tag="6")]
        pub text: ::prost::alloc::string::String,
    }
    /// Represent a request to fetch a raw object
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ComicRequest {
        /// The vector ID to be fetch.
        #[prost(uint64, tag="1")]
        pub id: u64,
    }
    /// Represent the Comic and distance pair.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Similarity {
        /// The Comic.
        #[prost(message, optional, tag="1")]
        pub comic: ::core::option::Option<Comic>,
        /// The distance from the prompt to this comic.
        #[prost(float, tag="2")]
        pub distance: f32,
    }
}
/// Represent an empty message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {
}
/// Generated server implementations.
pub mod xkcd_explorer_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with XkcdExplorerServer.
    #[async_trait]
    pub trait XkcdExplorer: Send + Sync + 'static {
        async fn exists(
            &self,
            request: tonic::Request<super::object::ComicRequest>,
        ) -> Result<tonic::Response<super::ExistsResponse>, tonic::Status>;
        async fn get(
            &self,
            request: tonic::Request<super::object::ComicRequest>,
        ) -> Result<tonic::Response<super::object::Comic>, tonic::Status>;
        async fn search(
            &self,
            request: tonic::Request<super::search::Request>,
        ) -> Result<tonic::Response<super::search::Response>, tonic::Status>;
        async fn insert(
            &self,
            request: tonic::Request<super::object::Comic>,
        ) -> Result<tonic::Response<super::Empty>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct XkcdExplorerServer<T: XkcdExplorer> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: XkcdExplorer> XkcdExplorerServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for XkcdExplorerServer<T>
    where
        T: XkcdExplorer,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/xkcd_explorer.v1.XkcdExplorer/Exists" => {
                    #[allow(non_camel_case_types)]
                    struct ExistsSvc<T: XkcdExplorer>(pub Arc<T>);
                    impl<
                        T: XkcdExplorer,
                    > tonic::server::UnaryService<super::object::ComicRequest>
                    for ExistsSvc<T> {
                        type Response = super::ExistsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::object::ComicRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).exists(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExistsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/xkcd_explorer.v1.XkcdExplorer/Get" => {
                    #[allow(non_camel_case_types)]
                    struct GetSvc<T: XkcdExplorer>(pub Arc<T>);
                    impl<
                        T: XkcdExplorer,
                    > tonic::server::UnaryService<super::object::ComicRequest>
                    for GetSvc<T> {
                        type Response = super::object::Comic;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::object::ComicRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/xkcd_explorer.v1.XkcdExplorer/Search" => {
                    #[allow(non_camel_case_types)]
                    struct SearchSvc<T: XkcdExplorer>(pub Arc<T>);
                    impl<
                        T: XkcdExplorer,
                    > tonic::server::UnaryService<super::search::Request>
                    for SearchSvc<T> {
                        type Response = super::search::Response;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::search::Request>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).search(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SearchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/xkcd_explorer.v1.XkcdExplorer/Insert" => {
                    #[allow(non_camel_case_types)]
                    struct InsertSvc<T: XkcdExplorer>(pub Arc<T>);
                    impl<
                        T: XkcdExplorer,
                    > tonic::server::UnaryService<super::object::Comic>
                    for InsertSvc<T> {
                        type Response = super::Empty;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::object::Comic>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).insert(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InsertSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: XkcdExplorer> Clone for XkcdExplorerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: XkcdExplorer> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: XkcdExplorer> tonic::transport::NamedService for XkcdExplorerServer<T> {
        const NAME: &'static str = "xkcd_explorer.v1.XkcdExplorer";
    }
}
