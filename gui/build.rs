fn main() {
    if cfg!(target_os = "windows") {
        winresource::WindowsResource::new()
            .set_icon("../tui/res/windows/icon.ico")
            .set_manifest_file("windows-xp.manifest")
            .compile()
            .expect("failed to embed Windows manifest");
    }
}
