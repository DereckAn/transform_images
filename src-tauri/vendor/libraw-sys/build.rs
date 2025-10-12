// Build script minimalista para macOS
// NO enlaza automáticamente ninguna librería C++
// El proyecto principal (transform-images) maneja todo el linking

use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    // Determinar si usar enlace estático o dinámico
    // Cuando usas features = ["static"] en Cargo.toml, Cargo establece CARGO_FEATURE_STATIC
    let is_static = env::var("CARGO_FEATURE_STATIC").is_ok();

    match target_os.as_str() {
        "macos" => configure_macos(is_static),
        "linux" => configure_linux(is_static),
        "windows" => configure_windows(is_static),
        _ => panic!("Unsupported platform: {}", target_os),
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_STATIC");
}

fn configure_macos(is_static: bool) {
    // macOS usa libc++ (LLVM), NO libstdc++ (GNU)
    println!("cargo:rustc-link-lib=dylib=c++");

    // Detectar arquitectura para rutas de Homebrew
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let homebrew_prefix = if arch == "aarch64" {
        "/opt/homebrew" // Apple Silicon (M1/M2/M3)
    } else {
        "/usr/local" // Intel
    };

    if is_static {
        println!("cargo:warning=🔗 LibRaw: Enlace ESTÁTICO (binario autocontenido)");

        // Rutas de búsqueda para bibliotecas estáticas
        println!("cargo:rustc-link-search=native={}/lib", homebrew_prefix);

        // Enlazar LibRaw estáticamente
        println!("cargo:rustc-link-lib=static=raw_r");

        // Enlazar lcms2 estáticamente (dependencia de LibRaw)
        println!("cargo:rustc-link-lib=static=lcms2");

        // Enlazar JPEG estáticamente
        println!("cargo:rustc-link-lib=static=jpeg");

        // Dependencias del sistema (vienen con macOS, NO necesitan instalación)
        println!("cargo:rustc-link-lib=dylib=z"); // zlib (compresión)
        println!("cargo:rustc-link-lib=dylib=iconv"); // conversión de caracteres
    } else {
        println!("cargo:warning=🔗 LibRaw: Enlace DINÁMICO (modo desarrollo)");

        // Enlace dinámico (para desarrollo rápido)
        println!("cargo:rustc-link-search=native={}/lib", homebrew_prefix);
        println!(
            "cargo:rustc-link-search=native={}/Cellar/libraw/0.21.4/lib",
            homebrew_prefix
        );
        println!(
            "cargo:rustc-link-search=native={}/opt/little-cms2/lib",
            homebrew_prefix
        );

        println!("cargo:rustc-link-lib=dylib=raw_r");
        println!("cargo:rustc-link-lib=dylib=lcms2");
    }
}

fn configure_linux(is_static: bool) {
    // Linux usa libstdc++ (GNU)
    println!("cargo:rustc-link-lib=dylib=stdc++");

    if is_static {
        println!("cargo:rustc-link-lib=static=raw_r");
        println!("cargo:rustc-link-lib=static=lcms2");
        println!("cargo:rustc-link-lib=static=jpeg");
    } else {
        println!("cargo:rustc-link-lib=dylib=raw_r");
        println!("cargo:rustc-link-lib=dylib=lcms2");
    }
}

fn configure_windows(is_static: bool) {
    use std::path::Path;

    if is_static {
        println!("cargo:warning=🔗 LibRaw: Enlace ESTÁTICO (binario autocontenido)");

        // Buscar bibliotecas en vcpkg o en directorio personalizado
        let vcpkg_root = env::var("VCPKG_ROOT")
            .or_else(|_| env::var("VCPKG_INSTALLATION_ROOT"))
            .unwrap_or_else(|_| {
                // Ubicación por defecto en GitHub Actions
                "C:\\vcpkg".to_string()
            });

        let target = env::var("TARGET").unwrap();
        let vcpkg_triplet = if target.contains("x86_64") {
            "x64-windows-static"
        } else if target.contains("i686") {
            "x86-windows-static"
        } else if target.contains("aarch64") {
            "arm64-windows-static"
        } else {
            "x64-windows-static"
        };

        let lib_path = format!("{}\\installed\\{}\\lib", vcpkg_root, vcpkg_triplet);

        // Verificar que las bibliotecas existen
        let raw_lib = Path::new(&lib_path).join("raw_r.lib");
        let lcms2_lib = Path::new(&lib_path).join("lcms2.lib");
        let jpeg_lib = Path::new(&lib_path).join("jpeg.lib");

        if raw_lib.exists() && lcms2_lib.exists() && jpeg_lib.exists() {
            println!("cargo:warning=✓ Bibliotecas estáticas encontradas en vcpkg");
            println!("cargo:rustc-link-search=native={}", lib_path);

            // Enlazar bibliotecas estáticamente
            println!("cargo:rustc-link-lib=static=raw_r");
            println!("cargo:rustc-link-lib=static=lcms2");
            println!("cargo:rustc-link-lib=static=jpeg");

            // Dependencias del sistema de Windows
            println!("cargo:rustc-link-lib=dylib=ws2_32");
            println!("cargo:rustc-link-lib=dylib=userenv");
        } else {
            println!("cargo:warning=❌ Bibliotecas NO encontradas en: {}", lib_path);
            println!("cargo:warning=   Esperando: raw_r.lib, lcms2.lib, jpeg.lib");
            println!("cargo:warning=   Instala con: vcpkg install libraw:x64-windows-static lcms:x64-windows-static libjpeg-turbo:x64-windows-static");
            panic!("LibRaw libraries not found. Please install via vcpkg.");
        }
    } else {
        println!("cargo:warning=🔗 LibRaw: Enlace DINÁMICO (modo desarrollo)");

        // Enlace dinámico para desarrollo
        println!("cargo:rustc-link-lib=dylib=raw");
        println!("cargo:rustc-link-lib=dylib=lcms2");
        println!("cargo:rustc-link-lib=dylib=jpeg");
    }
}
