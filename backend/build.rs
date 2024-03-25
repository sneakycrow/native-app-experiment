fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().compile(
        &[
            "protos/todo.proto",
            "protos/list.proto",
            "protos/user.proto",
        ],
        &["protos"],
    )?;
    Ok(())
}
