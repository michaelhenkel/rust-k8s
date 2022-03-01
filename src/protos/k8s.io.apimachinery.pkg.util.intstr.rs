/// IntOrString is a type that can hold an int32 or a string.  When used in
/// JSON or YAML marshalling and unmarshalling, it produces or consumes the
/// inner type.  This allows you to have, for example, a JSON field that can
/// accept a name or number.
/// TODO: Rename to Int32OrString
///
/// +protobuf=true
/// +protobuf.options.(gogoproto.goproto_stringer)=false
/// +k8s:openapi-gen=true
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntOrString {
    #[prost(int64, optional, tag="1")]
    #[serde(default)]
    pub r#type: ::core::option::Option<i64>,
    #[prost(int32, optional, tag="2")]
    #[serde(default)]
    pub int_val: ::core::option::Option<i32>,
    #[prost(string, optional, tag="3")]
    #[serde(default)]
    pub str_val: ::core::option::Option<::prost::alloc::string::String>,
}
