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

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    println!("cargo:warning=🔗 LibRaw: Linux {} (compilación nativa)", target_arch);

    if is_static {
        println!("cargo:rustc-link-lib=static=raw_r");
        println!("cargo:rustc-link-lib=static=lcms2");
        println!("cargo:rustc-link-lib=static=jpeg");
    } else {
        println!("cargo:rustc-link-lib=dylib=raw_r");
        println!("cargo:rustc-link-lib=dylib=lcms2");
    }
}

fn configure_windows(_is_static: bool) {
    use std::path::Path;

    println!("cargo:warning=🔗 LibRaw: Windows siempre usa enlace ESTÁTICO (autocontenido)");

    // Obtener el directorio del paquete (vendor/libraw-sys)
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = Path::new(&manifest_dir);

    // Arquitectura del target
    let target = env::var("TARGET").unwrap();
    let arch = if target.contains("aarch64") {
        "arm64"
    } else if target.contains("x86_64") {
        "x64"
    } else {
        "x64" // Por defecto
    };

    // Ruta a las bibliotecas dentro del proyecto (vendor/libraw-sys/libs/windows/x64)
    let lib_path = manifest_path.join("libs").join("windows").join(arch);

    println!("cargo:warning=📂 Buscando bibliotecas en: {}", lib_path.display());

    // Verificar que las bibliotecas existen
    let required_libs = vec![
        ("raw_r.lib", "LibRaw"),
        ("lcms2.lib", "Little CMS 2"),
        ("jpeg.lib", "JPEG Turbo"),
        ("zlib.lib", "zlib"),
    ];

    let mut all_found = true;
    let mut missing_libs = Vec::new();

    for (lib_file, lib_name) in &required_libs {
        let lib_full_path = lib_path.join(lib_file);
        if lib_full_path.exists() {
            println!("cargo:warning=  ✓ {} encontrado", lib_name);
        } else {
            println!("cargo:warning=  ✗ {} NO encontrado ({})", lib_name, lib_file);
            all_found = false;
            missing_libs.push(*lib_file);
        }
    }

    if !all_found {
        println!("cargo:warning=");
        println!("cargo:warning=❌ ERROR: Faltan bibliotecas de Windows en el proyecto");
        println!("cargo:warning=");
        println!("cargo:warning=Archivos faltantes:");
        for lib in &missing_libs {
            println!("cargo:warning=  - {}", lib);
        }
        println!("cargo:warning=");
        println!("cargo:warning=📖 Lee el archivo libs/windows/README.md para instrucciones");
        println!("cargo:warning=   sobre cómo obtener estas bibliotecas.");
        println!("cargo:warning=");
        println!("cargo:warning=Opción rápida:");
        println!("cargo:warning=  1. Instala vcpkg temporalmente:");
        println!("cargo:warning=     vcpkg install libraw:x64-windows-static lcms:x64-windows-static libjpeg-turbo:x64-windows-static zlib:x64-windows-static");
        println!("cargo:warning=");
        println!("cargo:warning=  2. Copia las bibliotecas al proyecto:");
        println!("cargo:warning=     Copy-Item \"C:\\vcpkg\\installed\\x64-windows-static\\lib\\*.lib\" -Destination \"{}\"", lib_path.display());
        println!("cargo:warning=");
        panic!(
            "LibRaw libraries not found in project. Expected path: {}",
            lib_path.display()
        );
    }

    // Si todas las bibliotecas existen, configurar el enlace
    println!("cargo:warning=✅ Todas las bibliotecas encontradas");
    println!("cargo:rustc-link-search=native={}", lib_path.display());

    // Enlazar bibliotecas estáticamente
    println!("cargo:rustc-link-lib=static=raw_r");
    println!("cargo:rustc-link-lib=static=lcms2");
    println!("cargo:rustc-link-lib=static=jpeg");
    println!("cargo:rustc-link-lib=static=zlib");

    // Dependencias del sistema de Windows (vienen con Windows)
    println!("cargo:rustc-link-lib=dylib=ws2_32"); // Windows Sockets
    println!("cargo:rustc-link-lib=dylib=userenv"); // User Environment
}
