use anyhow::Result;

fn main() -> Result<()> {
    prost_build::Config::new()
        .bytes(&["."])
        // .type_attribute(".", "#[derive(PartialOrd)]")
        .out_dir("src/pb")
        .compile_protos(&["user.proto", "blog.proto"], &["."])?;

    Ok(())
}
