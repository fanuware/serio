use vergen_git2::{BuildBuilder, Emitter, Git2Builder, RustcBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/heads");
    println!("cargo:rerun-if-changed=.git/refs/tags");
    let build = BuildBuilder::default().build_timestamp(true).build()?;

    let git2 = Git2Builder::default().sha(true).dirty(true).describe(true, true, None).build()?;

    let rustc = RustcBuilder::default().semver(true).build()?;

    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&git2)?
        .add_instructions(&rustc)?
        .emit()?;

    Ok(())
}
