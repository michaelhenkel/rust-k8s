use futures::prelude::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::OwnerReference;
use kube::{
    api::{Api, DynamicObject, GroupVersionKind, ListParams, ResourceExt, PostParams},
    discovery::{verbs, Discovery, Scope},
    discovery,
    runtime::{utils::try_flatten_applied, watcher},
    Client, core::object::HasSpec,
};

use serde::{Serialize, Deserialize};

use std::env;
use prost::Message;
use std::io::Cursor;
use kube_client::protos;
use kube_client::protos::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1;
pub mod lib{
    pub mod items{

    }
}

pub fn serialize_virtual_network(vn: &protos::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1::VirtualNetwork) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(vn.encoded_len());

    vn.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_virtual_network(buf: &[u8]) -> Result<protos::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1::VirtualNetwork, prost::DecodeError> {
    protos::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1::VirtualNetwork::decode(&mut Cursor::new(buf))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info,kube=debug");
    env_logger::init();
    let client = Client::try_default().await?;
    let group_filter: [&str; 1] = ["core.contrail.juniper.net"];
    let disco = Discovery::new(client.clone()).filter(&group_filter).run().await?;
    /*
    let disco = Discovery::filter(disco, &group_filter);
    let disco = disco.run().await?;
    */
    //let disco = Discovery::new(client.clone()).run().await?;
    
    for group in disco.groups() {
        for (ar, _caps) in group.recommended_resources(){
            if ar.kind == "VirtualNetwork" {
                //let api_x: Api<protos::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1::VirtualNetwork> = Api::all_with(client.clone(), &ar);
                let api: Api<DynamicObject> = Api::all_with(client.clone(), &ar);
                let list = api.list(&Default::default()).await?;
                for item in list.items {
                    //item.metadata.owner_references
                    let or = Some(vec![OwnerReference::default()]);
                    let mut new_item = item.clone();
                    new_item.metadata.owner_references = or;
                    let json_string = serde_json::to_string(&new_item).unwrap();
                    let spec_data = &item.data["spec"];
                    let spec_json_string = serde_json::to_string(&spec_data).unwrap();
                    //let vn = deserialize_virtual_network(&ser);
                    //item.serialize(serialize_virtual_netwrk);
                    //item.metadata.owner_references.
                    println!("{}", json_string);
                    //let vn = v1alpha1::VirtualNetwork::default();
                    //let vn_spec = vn.spec.unwrap();
                    //let vn_string = serde_json::to_string(&vn.spec).unwrap();
                    //println!("{}", vn_string);
                    
                    let vn_x: v1alpha1::VirtualNetworkSpec = serde_json::from_str(&spec_json_string)?;
                    let vnx_string = serde_json::to_string(&vn_x).unwrap();
                    println!("{}", spec_json_string);
                    println!("{}", vnx_string);

                    //md.owner_references
                    //let vn: protos::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1::VirtualNetworkSpec = serde_json::from_str(&json_string).unwrap();
                    //log::info!("{:#?}", vn);
                    //let vn = protos::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1::VirtualNetwork::default();
                    //let serialized_vn = serialize_virtual_network(&vn);
                    //serde_json::to_string(&vn).unwrap();
                    //let vn = protos::ssd_git_juniper_net_contrail_cn2_contrail_pkg_apis_core_v1alpha1_generated::VirtualNetwork::new();

                    //let json_string = serde_json::to_string(&item);
                    
                    //let vn: protos::ssd_git_juniper_net_contrail_cn2_contrail_pkg_apis_core_v1alpha1_generated::VirtualNetwork = protobuf::Message::parse_from_bytes(&json_byte.unwrap());
                    
                    //let data= item.data;
                    //let spec = data["spec"].as_object().unwrap();
                    //let vnSpec = protos::ssd_git_juniper_net_contrail_cn2_contrail_pkg_apis_core_v1alpha1_generated::VirtualNetworkSpec::new();
                    
                }
            }
            /*
            let api = Api::<DynamicObject>::all_with(client.clone(), &ar);
            let res = api.get(&"bla");
            //let vns1: Api<VirtualNetwork> = Api::all(client);
            //let vns = Api::<protos::ssd_git_juniper_net_contrail_cn2_contrail_pkg_apis_core_v1alpha1_generated::VirtualNetwork>::namespaced(client, &"bla");
            let vn = protos::ssd_git_juniper_net_contrail_cn2_contrail_pkg_apis_core_v1alpha1_generated::VirtualNetwork::new();
            let md = vn.get_metadata();
            vns.create(&PostParams::default, &vn);
            //log::info!("{}/{} : {}", group.name(), ar.version, ar.kind);
            */
        }
    }

    /*
    // Take dynamic resource identifiers:
    let group = env::var("GROUP").unwrap_or_else(|_| "core.contrail.juniper.net".into());
    let version = env::var("VERSION").unwrap_or_else(|_| "v1alpha1".into());
    let kind = env::var("KIND").unwrap_or_else(|_| "VirtualNetwork".into());

    // Turn them into a GVK
    let gvk = GroupVersionKind::gvk(&group, &version, &kind);
    // Use API discovery to identify more information about the type (like its plural)
    let (ar, _caps) = discovery::pinned_kind(&client, &gvk).await?;

    // Use the discovered kind in an Api with the ApiResource as its DynamicType
    let api = Api::<DynamicObject>::all_with(client, &ar);

    // Fully compatible with kube-runtime
    try_flatten_applied(watcher(api, ListParams::default()))
        .try_for_each(|p| async move {
            log::info!("Applied: {}", p.name());
            Ok(())
        })
        .await?;
    */
    Ok(())
}