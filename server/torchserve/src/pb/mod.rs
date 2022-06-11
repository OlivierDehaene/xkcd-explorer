pub mod org {
    pub mod pytorch {
        pub mod serve {
            pub mod grpc {
                pub mod inference {
                    include!("org.pytorch.serve.grpc.inference.rs");
                }
            }
        }
    }
}
pub mod google {
    pub mod protobuf {
        include!("google.protobuf.rs");
    }
}
