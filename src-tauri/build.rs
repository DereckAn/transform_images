use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // 1. Tauri build (necesario para Tauri)
    tauri_build::build();

    // 2. Verificar e instalar LibRaw
    setup_libraw();

    // 3. Configurar linking
    configure_libraw_linking();
}

// Función principal para setup de LibRaw
fn setup_libraw() {
    // Detectar sistema operativo
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    println!("cargo:warning=Detecting OS: {}", os);

    match os.as_str() {
        "macos" => setup_libraw_macos(),
        "linux" => setup_libraw_linux(),
        "windows" => setup_libraw_windows(),
        _ => {
            println!("cargo:warning=Unknown OS: {}. Skipping libRaw setup.", os);
        }
    }
}

// Setup para macOS
fn setup_libraw_macos() {
    println!("cargo:warning=Setting up libRaw for macOS...");

    // Verificar si libraw esta instalado con pkg-config
    let pkg_config_result = Command::new("pkg-config")
        .args(&["--exists", "libraw"])
        .status();

    match pkg_config_result {
        Ok(status) if status.success() => {
            println!("cargo:warning=LibRaw already installed ✓");
        }
        _ => {
            println!("cargo:warning=LibRaw not found. Attempting to install with Homebrew...");
            install_libraw_homebrew();
        }
    }
}

// Instalar LibRaw con Homebrew
fn install_libraw_homebrew() {
    // Verificar si Homebrew está instalado
    let brew_check = Command::new("which").arg("brew").output();

    match brew_check {
        Ok(output) if output.status.success() => {
            println!("cargo:warning=Homebrew found. Installing LibRaw...");

            println!("cargo:warning=Updating Homebrew...");
            let _ = Command::new("brew").arg("update").status();

            println!("cargo:warning=Installing/updating LibRaw to latest version...");
            let install_result = Command::new("brew").args(&["install", "libraw"]).status();

            match install_result {
                Ok(status) if status.success() => {
                    println!("cargo:warning=LibRaw installed successfully ✓");
                }
                _ => {
                    panic!(
                        "Failed to install LibRaw with Homebrew. Please install manually: brew install libraw"
                    );
                }
            }
        }
        _ => {
            panic!(
                "Homebrew not found. Please install Homebrew first or install LibRaw manually: brew install libraw"
            );
        }
    }
}

// Placeholder para Linux
fn setup_libraw_linux() {
    println!("cargo:warning=Linux detected. Please ensure libraw is installed:");
    println!("cargo:warning=  Ubuntu/Debian: sudo apt install libraw-dev");
    println!("cargo:warning=  Fedora: sudo dnf install libraw-devel");
    println!("cargo:warning=  Arch: sudo pacman -S libraw");
}

// Placeholder para Windows
fn setup_libraw_windows() {
    println!("cargo:warning=Windows detected. LibRaw setup for Windows is complex.");
    println!("cargo:warning=Please follow instructions at: https://www.libraw.org/download");
}

fn configure_libraw_linking() {
    println!("cargo:warning=Configuring LibRaw linking...");

    // Detectar sistema operativo
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    match os.as_str() {
        "macos" => {
            // macOS usa libc++ (LLVM), NO libstdc++ (GNU)
            println!("cargo:rustc-link-lib=dylib=c++");

            // Rutas de LibRaw en Homebrew
            let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
            let homebrew_prefix = if arch == "aarch64" {
                "/opt/homebrew" // Apple Silicon (M1/M2/M3)
            } else {
                "/usr/local" // Intel
            };

            println!("cargo:rustc-link-search=native={}/lib", homebrew_prefix);
            println!(
                "cargo:rustc-link-search=native={}/Cellar/libraw/0.21.4/lib",
                homebrew_prefix
            );
            println!(
                "cargo:rustc-link-search=native={}/opt/little-cms2/lib",
                homebrew_prefix
            );

            // Enlazar LibRaw (versión thread-safe)
            println!("cargo:rustc-link-lib=dylib=raw_r");

            // Enlazar lcms2 (dependencia de LibRaw)
            println!("cargo:rustc-link-lib=dylib=lcms2");

            println!("cargo:warning=LibRaw linking configured for macOS with libc++");
        }
        "linux" => {
            // Linux usa libstdc++ (GNU)
            println!("cargo:rustc-link-lib=dylib=stdc++");
            println!("cargo:rustc-link-lib=dylib=raw");
            println!("cargo:rustc-link-lib=dylib=raw_r");
            println!("cargo:warning=LibRaw linking configured for Linux");
        }
        "windows" => {
            // Windows linking (si alguna vez se necesita)
            println!("cargo:warning=Windows LibRaw linking not configured");
        }
        _ => {
            println!("cargo:warning=Unknown OS for LibRaw linking");
        }
    }
}
