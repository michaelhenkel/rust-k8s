use futures::prelude::*;
use kube_client::*;
use kube::{
    api::{Api, DynamicObject, GroupVersionKind, ListParams, ResourceExt},
    discovery::{verbs, Discovery, Scope},
    discovery,
    runtime::{utils::try_flatten_applied, watcher},
    Client,
};

use std::env;

mod protos {
    pub mod ssd_git_juniper_net_contrail_cn2_contrail_pkg_apis_core_v1alpha1_generated;
    pub mod k8s_io_apimachinery_pkg_runtime_generated;
    pub mod k8s_io_apimachinery_pkg_apis_meta_v1_generated;
    pub mod k8s_io_apimachinery_pkg_api_resource_generated;
    pub mod k8s_io_apimachinery_pkg_util_intstr_generated;
    pub mod k8s_io_api_core_v1_generated;
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let vn = protos::ssd_git_juniper_net_contrail_cn2_contrail_pkg_apis_core_v1alpha1_generated::VirtualNetwork::new();
    let vn_spec = vn.get_spec();
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
            //log::info!("{}/{} : {}", group.name(), ar.version, ar.kind);
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