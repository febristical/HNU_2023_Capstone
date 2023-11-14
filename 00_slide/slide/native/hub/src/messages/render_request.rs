// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenderRequest {
    #[prost(double, tag="1")]
    pub angle: f64,
    #[prost(uint32, tag="2")]
    pub style: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenderResponse {
    #[prost(bytes="vec", tag="1")]
    pub image: ::prost::alloc::vec::Vec<u8>,
}
// @@protoc_insertion_point(module)

pub const ID: i32 = 4;