fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../protos/herro.proto")?;
    tonic_build::compile_protos("../protos/admin.proto")?;
    Ok(())
}
