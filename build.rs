fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::Config::new()
    .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
    .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
    .field_attribute("time", "#[serde(skip)]")  
    .field_attribute(".", "#[serde(default)]")
    .out_dir("src/protos")
    .include_file("mod.rs")
    .compile_protos(
        &["src/protos/ssd-git.juniper.net/contrail/cn2/contrail/pkg/apis/core/v1alpha1/generated.proto"],
        &["src/protos/"],
    )?;

/*   
    tonic_build::configure()
    .build_server(false)
    .out_dir("src/protos")
    .disable_package_emission()
    .include_file("mod.rs")
    .compile_with_config(prost_build::Config::new()
    .type_attribute(path, attribute)
    , &["src/protos/ssd-git.juniper.net/contrail/cn2/contrail/pkg/apis/core/v1alpha1/generated.proto"], 
    &["src/protos/"])
    .compile(
        &["src/protos/ssd-git.juniper.net/contrail/cn2/contrail/pkg/apis/core/v1alpha1/generated.proto"],
        &["src/protos/"],
    )?;
*/  
    Ok(())
}