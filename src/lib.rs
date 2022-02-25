use std::io::Cursor;

use prost::Message;

#[path = "protos/mod.rs"] mod items;

// Include the `items` module, which is generated from items.proto.
//pub mod items {
//    include!(concat!(env!("OUT_DIR"), "/k8s.io.apimachinery.pkg.apis.meta.v1.rs"));
//}

pub fn test(){
    let bla = items::k8s::io::apimachinery::pkg::apis::meta::v1::GetOptions::default();
}