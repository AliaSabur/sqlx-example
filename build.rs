use std::process::Command;

use static_files::resource_dir;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=MarkVuePress/src");
    Command::new("npm")
        .args(&["run", "build"])
        .current_dir("MarkVuePress")
        .status()?;
    resource_dir("./MarkVuePress/dist").build()?;
    Ok(())
}
