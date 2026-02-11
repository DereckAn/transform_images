// Build script para LibRaw con soporte multi-arquitectura
// Soporta: macOS (Intel + Apple Silicon), Linux (x64 + ARM64), Windows (x64 + ARM64)

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target = env::var("TARGET").unwrap();

    // Determinar si usar enlace estático o dinámico
    let is_static = env::var("CARGO_FEATURE_STATIC").is_ok();

    println!("cargo:warning=🔧 Building for: {} {} (target: {})", target_os, target_arch, target);

    match target_os.as_str() {
        "macos" => configure_macos(is_static, &target_arch, &target),
        "linux" => configure_linux(is_static, &target_arch),
        "windows" => configure_windows(&target),
        _ => panic!("Unsupported platform: {}", target_os),
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_STATIC");
    println!("cargo:rerun-if-env-changed=HOMEBREW_PREFIX");
}

fn configure_macos(is_static: bool, target_arch: &str, target: &str) {
    // macOS usa libc++ (LLVM), NO libstdc++ (GNU)
    println!("cargo:rustc-link-lib=dylib=c++");

    // LibRaw se compila con soporte OpenMP, necesitamos enlazar libomp
    println!("cargo:rustc-link-lib=dylib=omp");

    // Determinar Homebrew prefix basado en la arquitectura TARGET (no host)
    // IMPORTANTE: Esto maneja correctamente cross-compilation
    let homebrew_prefix = if target.contains("aarch64") || target_arch == "aarch64" {
        "/opt/homebrew" // Apple Silicon
    } else {
        "/usr/local" // Intel
    };

    println!("cargo:warning=📦 Homebrew prefix: {}", homebrew_prefix);

    // Intentar detectar rutas alternativas de Homebrew
    let alternate_paths = get_homebrew_lib_paths(homebrew_prefix, target_arch);

    if is_static {
        println!("cargo:warning=🔗 LibRaw: Enlace ESTÁTICO (binario autocontenido)");

        // Agregar rutas de búsqueda para bibliotecas estáticas
        println!("cargo:rustc-link-search=native={}/lib", homebrew_prefix);

        // Agregar rutas alternativas detectadas
        for path in &alternate_paths {
            println!("cargo:rustc-link-search=native={}", path);
        }

        // Intentar usar pkg-config para encontrar las bibliotecas
        if let Ok(lib_path) = try_pkg_config("libraw") {
            println!("cargo:rustc-link-search=native={}", lib_path);
        }
        if let Ok(lib_path) = try_pkg_config("lcms2") {
            println!("cargo:rustc-link-search=native={}", lib_path);
        }
        if let Ok(lib_path) = try_pkg_config("libjpeg") {
            println!("cargo:rustc-link-search=native={}", lib_path);
        }

        // Enlazar bibliotecas estáticamente
        println!("cargo:rustc-link-lib=static=raw_r");
        println!("cargo:rustc-link-lib=static=lcms2");
        println!("cargo:rustc-link-lib=static=jpeg");

        // Dependencias del sistema (vienen con macOS)
        println!("cargo:rustc-link-lib=dylib=z");      // zlib
        println!("cargo:rustc-link-lib=dylib=iconv");  // iconv
    } else {
        println!("cargo:warning=🔗 LibRaw: Enlace DINÁMICO (modo desarrollo)");

        // Enlace dinámico para desarrollo
        println!("cargo:rustc-link-search=native={}/lib", homebrew_prefix);

        for path in &alternate_paths {
            println!("cargo:rustc-link-search=native={}", path);
        }

        println!("cargo:rustc-link-lib=dylib=raw_r");
        println!("cargo:rustc-link-lib=dylib=lcms2");
        println!("cargo:rustc-link-lib=dylib=jpeg");
    }

    // Verificar que las bibliotecas existen
    verify_macos_libs(homebrew_prefix, &alternate_paths, is_static);
}

fn get_homebrew_lib_paths(prefix: &str, _arch: &str) -> Vec<String> {
    let mut paths = Vec::new();

    // Paths comunes de Homebrew
    let common_packages = vec![
        "libraw",
        "little-cms2",
        "jpeg-turbo",
        "jpeg",
    ];

    for package in common_packages {
        // Buscar en opt (symlink a la versión actual)
        let opt_path = format!("{}/opt/{}/lib", prefix, package);
        if Path::new(&opt_path).exists() {
            paths.push(opt_path);
        }

        // Buscar en Cellar (versiones específicas)
        let cellar_base = format!("{}/Cellar/{}", prefix, package);
        if let Ok(entries) = std::fs::read_dir(&cellar_base) {
            for entry in entries.filter_map(|e| e.ok()) {
                let lib_path = entry.path().join("lib");
                if lib_path.exists() {
                    if let Some(path_str) = lib_path.to_str() {
                        paths.push(path_str.to_string());
                    }
                }
            }
        }
    }

    println!("cargo:warning=🔍 Found {} additional library paths", paths.len());
    paths
}

fn try_pkg_config(package: &str) -> Result<String, ()> {
    let output = Command::new("pkg-config")
        .args(&["--variable=libdir", package])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !path.is_empty() && Path::new(&path).exists() {
                println!("cargo:warning=✓ pkg-config found {}: {}", package, path);
                return Ok(path);
            }
            Err(())
        }
        _ => Err(()),
    }
}

fn verify_macos_libs(prefix: &str, extra_paths: &[String], is_static: bool) {
    let lib_ext = if is_static { ".a" } else { ".dylib" };
    let libs = vec![
        ("libraw_r", "libraw"),
        ("liblcms2", "little-cms2"),
        ("libjpeg", "jpeg-turbo"),
    ];

    let mut all_found = true;

    for (lib_name, package) in &libs {
        let lib_file = format!("{}{}", lib_name, lib_ext);
        let mut found = false;

        // Buscar en prefix/lib
        if Path::new(&format!("{}/lib/{}", prefix, lib_file)).exists() {
            found = true;
        }

        // Buscar en paths extra
        if !found {
            for path in extra_paths {
                if Path::new(&format!("{}/{}", path, lib_file)).exists() {
                    found = true;
                    break;
                }
            }
        }

        if found {
            println!("cargo:warning=✓ {} encontrado", lib_name);
        } else {
            println!("cargo:warning=❌ {} NO encontrado", lib_name);
            println!("cargo:warning=   Instala con: brew install {}", package);
            all_found = false;
        }
    }

    if !all_found {
        println!("cargo:warning=");
        println!("cargo:warning=⚠️ ADVERTENCIA: Algunas bibliotecas no se encontraron");
        println!("cargo:warning=   La compilación puede fallar");
        println!("cargo:warning=   Ejecuta: brew install libraw little-cms2 jpeg-turbo");
    }
}

fn configure_linux(is_static: bool, target_arch: &str) {
    // Linux usa libstdc++ (GNU)
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:warning=🔗 LibRaw: Linux {} (compilación nativa)", target_arch);

    if is_static {
        println!("cargo:rustc-link-lib=static=raw_r");
        println!("cargo:rustc-link-lib=static=lcms2");
        println!("cargo:rustc-link-lib=static=jpeg");
    } else {
        println!("cargo:rustc-link-lib=dylib=raw_r");
        println!("cargo:rustc-link-lib=dylib=lcms2");
        println!("cargo:rustc-link-lib=dylib=jpeg");
    }
}

fn configure_windows(target: &str) {
    println!("cargo:warning=🔗 LibRaw: Windows enlace ESTÁTICO");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = Path::new(&manifest_dir);

    let arch = if target.contains("aarch64") {
        "arm64"
    } else {
        "x64"
    };

    let lib_path = manifest_path.join("libs").join("windows").join(arch);
    println!("cargo:warning=📂 Buscando bibliotecas en: {}", lib_path.display());

    let required_libs = vec![
        ("raw_r.lib", "LibRaw"),
        ("lcms2.lib", "Little CMS 2"),
        ("jpeg.lib", "JPEG Turbo"),
        ("zlib.lib", "zlib"),
    ];

    let mut all_found = true;

    for (lib_file, lib_name) in &required_libs {
        if lib_path.join(lib_file).exists() {
            println!("cargo:warning=  ✓ {} encontrado", lib_name);
        } else {
            println!("cargo:warning=  ✗ {} NO encontrado", lib_name);
            all_found = false;
        }
    }

    if !all_found {
        panic!("Bibliotecas de Windows no encontradas en: {}", lib_path.display());
    }

    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=static=raw_r");
    println!("cargo:rustc-link-lib=static=lcms2");
    println!("cargo:rustc-link-lib=static=jpeg");
    println!("cargo:rustc-link-lib=static=zlib");
    println!("cargo:rustc-link-lib=dylib=ws2_32");
    println!("cargo:rustc-link-lib=dylib=userenv");
}
