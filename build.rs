#[cfg(target_os = "windows")]
fn main() {
    embed_resource::compile("./static/icon/icon.rc", embed_resource::NONE).manifest_optional().unwrap();
}

#[cfg(not(target_os = "windows"))]
fn main() {}
