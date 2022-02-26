use std::io::Cursor;

use prost::Message;

#[path = "protos/mod.rs"] pub mod protos;

// Include the `items` module, which is generated from items.proto.
//pub mod items {
//    include!(concat!(env!("OUT_DIR"), "/k8s.io.apimachinery.pkg.apis.meta.v1.rs"));
//}
