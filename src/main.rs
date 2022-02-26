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
                    //let vn = deserialize_virtual_network(&ser);
                    //item.serialize(serialize_virtual_netwrk);
                    //item.metadata.owner_references.
                    println!("{}", json_string);
                    let vn = v1alpha1::VirtualNetwork::default();
                    //let vn_spec = vn.spec.unwrap();
                    let vn_string = serde_json::to_string(&vn).unwrap();
                    println!("{}", vn_string);
                    let data = r#"
                    {
                        "metadata": {
                            "annotations": {
                                "juniper.net/nad-cluster-name": "contrail-k8s-kubemanager-cluster1-local",
                                "juniper.net/nad-name": "ocmh-1",
                                "juniper.net/nad-namespace": "ocmh"
                            },
                            "labels": {},
                            "owner_references": []
                        },
                        "spec": null,
                        "status": null
                    }"#;
                    let data_x = r#"
                    {
                        "metadata": {
                            "annotations": {
                                "juniper.net/nad-cluster-name": "contrail-k8s-kubemanager-cluster1-local",
                                "juniper.net/nad-name": "ocmh-1",
                                "juniper.net/nad-namespace": "ocmh"
                            },
                            "creationTimestamp": "2022-02-23T18:29:16Z",
                            "finalizers": ["virtual-network-id-deallocation.finalizers.core.juniper.net", "route-target.finalizers.core.juniper.net", "vn-routinginstance-delete.finalizers.core.juniper.net"],
                            "generation": 1,
                            "labels": {
                                "back-reference.core.juniper.net/347901e75899ab8a4cde2f0275a7883f24c7689777025c80c7ad0b8f": "Subnet_ocmh_ocmh-1-v4"
                            },
                            "managedFields": [{
                                "apiVersion": "core.contrail.juniper.net/v1alpha1",
                                "fieldsType": "FieldsV1",
                                "fieldsV1": {
                                    "f:metadata": {
                                        "f:annotations": {
                                            ".": {},
                                            "f:juniper.net/nad-cluster-name": {},
                                            "f:juniper.net/nad-name": {},
                                            "f:juniper.net/nad-namespace": {}
                                        }
                                    },
                                    "f:spec": {
                                        "f:fabricSNAT": {},
                                        "f:v4SubnetReference": {
                                            ".": {},
                                            "f:apiVersion": {},
                                            "f:kind": {},
                                            "f:name": {},
                                            "f:namespace": {},
                                            "f:resourceVersion": {},
                                            "f:uid": {}
                                        },
                                        "f:virtualNetworkProperties": {
                                            "f:forwardingMode": {},
                                            "f:rpf": {}
                                        }
                                    }
                                },
                                "manager": "kubemanager",
                                "operation": "Update",
                                "time": "2022-02-23T18:29:16Z"
                            }, {
                                "apiVersion": "core.contrail.juniper.net/v1alpha1",
                                "fieldsType": "FieldsV1",
                                "fieldsV1": {
                                    "f:metadata": {
                                        "f:finalizers": {
                                            ".": {},
                                            "v:\"route-target.finalizers.core.juniper.net\"": {},
                                            "v:\"virtual-network-id-deallocation.finalizers.core.juniper.net\"": {},
                                            "v:\"vn-routinginstance-delete.finalizers.core.juniper.net\"": {}
                                        }
                                    },
                                    "f:status": {
                                        "f:observation": {},
                                        "f:state": {},
                                        "f:virtualNetworkNetworkId": {}
                                    }
                                },
                                "manager": "manager",
                                "operation": "Update",
                                "time": "2022-02-23T18:29:16Z"
                            }],
                            "name": "ocmh-1",
                            "namespace": "ocmh",
                            "ownerReferences": [{
                                "apiVersion": "",
                                "kind": "",
                                "name": "",
                                "uid": ""
                            }],
                            "resourceVersion": "19355017",
                            "uid": "73d0a5b6-f906-45b8-b837-0a60fabcda52"
                        },
                        "spec": {
                            "fqName": ["default-domain", "ocmh", "ocmh-1"],
                            "fabricSNAT": false,
                            "v4SubnetReference": {
                                "kind": "Subnet",
                                "namespace": "ocmh",
                                "name": "ocmh-1-v4",
                                "uid": "d63a35bb-f48b-43e6-ac9e-ae165ecc7b18",
                                "apiVersion": "core.contrail.juniper.net/v1alpha1",
                                "resourceVersion": "19354960",
                                "fqName": ["default-domain", "ocmh", "ocmh-1-v4"]
                            },
                            "virtualNetworkProperties": {
                                "rpf": "enable",
                                "forwardingMode": "l2_l3"
                            }
                        },
                        "status": {
                            "state": "Success",
                            "observation": "",
                            "virtualNetworkNetworkId": 9
                        }
                    }"#;
                    let vn_x: v1alpha1::VirtualNetwork = serde_json::from_str(&data)?;

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