use std::io::Result;
fn main() -> Result<()> {
    println!("IN BUILD.RS");
    prost_build::compile_protos(&["src/items.proto"], &["src/"])?;
    Ok(())
}
