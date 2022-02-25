fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
    .build_server(false)
    .out_dir("src/protos")
    .disable_package_emission()
    .include_file("mod.rs")
    .compile(
        &["src/protos/k8s.io/apimachinery/pkg/apis/meta/v1/generated.proto"],
        &["src/protos/"],
    )?;
    //tonic_build::compile_protos("src/protos/k8s.io/apimachinery/pkg/apis/meta/v1/generated.proto")?;
    Ok(())
}