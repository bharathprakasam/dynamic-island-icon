fn main() {
    // Re-run build script if styles or assets change
    println!("cargo:rerun-if-changed=src/ui/styles.css");
    println!("cargo:rerun-if-changed=assets/");
}
