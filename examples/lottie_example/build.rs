fn main() {
    // Tell Cargo that if the .slint file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=main_ui.slint");
    println!("cargo:rerun-if-changed=../../lottie_player.slint"); 
    slint_build::compile("main_ui.slint").unwrap();
}
