// Build script minimalista para macOS
// NO enlaza automáticamente ninguna librería C++
// El proyecto principal (transform-images) maneja todo el linking

fn main() {
    // No hacer nada - el proyecto principal maneja el linking
    println!("cargo:rerun-if-changed=build.rs");
}
