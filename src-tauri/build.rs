use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // 1. Tauri build (necesario para Tauri)
    tauri_build::build();

    // 2. Detectar si estamos usando enlace estático
    // La feature "static" de libraw-sys se expone como CARGO_FEATURE_STATIC
    let is_static_build = env::var("CARGO_FEATURE_STATIC").is_ok();

    if is_static_build {
        verify_static_libraries();
    } else {
        verify_libraw_for_development();
    }
}

/// Verificación para desarrollo con enlace dinámico
fn verify_libraw_for_development() {
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    println!("cargo:warning=🔧 Modo DESARROLLO (enlace dinámico)");

    if os == "macos" {
        let pkg_config_result = Command::new("pkg-config")
            .args(&["--exists", "libraw"])
            .status();

        match pkg_config_result {
            Ok(status) if status.success() => {
                println!("cargo:warning=✓ LibRaw encontrado para desarrollo");
            }
            _ => {
                println!("cargo:warning=⚠️ LibRaw no encontrado.");
                println!("cargo:warning=   Instala con: brew install libraw");
                println!(
                    "cargo:warning=   O compila con: cargo build --release (usa enlace estático)"
                );
            }
        }
    }
}

/// Verificación para producción con enlace estático
fn verify_static_libraries() {
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    println!("cargo:warning=📦 Modo PRODUCCIÓN (enlace estático - binario autocontenido)");

    if os == "macos" {
        let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
        let homebrew_prefix = if arch == "aarch64" {
            "/opt/homebrew"
        } else {
            "/usr/local"
        };

        // Lista de bibliotecas estáticas requeridas
        let required_libs = vec![
            ("libraw_r.a", "libraw"),
            ("liblcms2.a", "little-cms2"),
            ("libjpeg.a", "jpeg-turbo"),
        ];

        let mut all_found = true;

        for (lib_file, brew_package) in required_libs {
            let lib_path = format!("{}/lib/{}", homebrew_prefix, lib_file);

            if !std::path::Path::new(&lib_path).exists() {
                println!("cargo:warning=❌ {} no encontrado", lib_file);
                println!(
                    "cargo:warning=   Instala con: brew install {}",
                    brew_package
                );
                all_found = false;
            } else {
                println!("cargo:warning=✓ {} encontrado", lib_file);
            }
        }

        if all_found {
            println!(
                "cargo:warning=🎉 Todas las bibliotecas estáticas OK - binario será autocontenido"
            );
        } else {
            println!("cargo:warning=");
            println!("cargo:warning=⚠️ FALTA INSTALAR ALGUNAS BIBLIOTECAS");
            println!("cargo:warning=   Ejecuta: brew install libraw little-cms2 jpeg-turbo");
        }
    }
}
