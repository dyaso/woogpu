// this file's here 'cos otherwise NixOS for some reason doesn't link to vulkan correctly? idk
// found it at https://github.com/gfx-rs/wgpu-rs/issues/332#issuecomment-655672840

fn main() {
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=X11");
        println!("cargo:rustc-link-lib=Xcursor");
        println!("cargo:rustc-link-lib=Xrandr");
        println!("cargo:rustc-link-lib=Xi");
        println!("cargo:rustc-link-lib=vulkan");
    }
}