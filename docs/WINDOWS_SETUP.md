# Solución del Error de Linking de LibRaw en Windows

**Fecha:** 13 de Enero, 2026  
**Problema:** Error de compilación `linking with link.exe` failed: exit code: 1181` en Windows  
**Proyecto:** QuakImages (Tauri + Rust)  
**Estado:** ✅ **RESUELTO** - Bibliotecas ahora incluidas en el proyecto

---

## 📋 Problema

Al intentar compilar el proyecto en Windows con `bun run tauri dev`, se produce un error de linking:

```
error: linking with `link.exe` failed: exit code: 1181
```

**Causa:** El linker de Windows no puede encontrar las bibliotecas estáticas de LibRaw necesarias para compilar el proyecto.

---

## ✅ Solución Implementada

### Enfoque: Proyecto Autocontenido

El proyecto ahora incluye las bibliotecas **dentro del repositorio**, lo que hace que sea completamente autocontenido:

```
src-tauri/
  vendor/
    libraw-sys/
      libs/
        windows/
          x64/
            raw_r.lib     ← LibRaw estática
            lcms2.lib     ← Little CMS 2
            jpeg.lib      ← JPEG Turbo
            zlib.lib      ← zlib
```

**Ventajas:**

- ✅ No requiere instalaciones externas (vcpkg, Homebrew, etc.)
- ✅ Funciona en cualquier máquina Windows sin configuración
- ✅ Los usuarios solo clonan y compilan
- ✅ Mismo enfoque que usas en macOS con tu vendor personalizado

---

## 🚀 Pasos para Desarrolladores

### Si las bibliotecas YA están en el proyecto

```bash
# Simplemente compila
bun run tauri dev
```

¡Eso es todo! 🎉

### Si las bibliotecas NO están (primera vez)

Necesitas obtener las bibliotecas `.lib` **una sola vez**:

#### Opción 1: Usar vcpkg temporalmente (Recomendado)

```powershell
# 1. Instalar vcpkg temporalmente
git clone https://github.com/Microsoft/vcpkg.git C:\vcpkg_temp
cd C:\vcpkg_temp
.\bootstrap-vcpkg.bat

# 2. Instalar las bibliotecas
.\vcpkg install libraw:x64-windows-static lcms:x64-windows-static libjpeg-turbo:x64-windows-static zlib:x64-windows-static

# 3. Copiar al proyecto
$projectPath = "C:\Users\derec\Documents\Git\transform_images"
$destPath = "$projectPath\src-tauri\vendor\libraw-sys\libs\windows\x64"

Copy-Item "C:\vcpkg_temp\installed\x64-windows-static\lib\raw_r.lib" -Destination $destPath
Copy-Item "C:\vcpkg_temp\installed\x64-windows-static\lib\lcms2.lib" -Destination $destPath
Copy-Item "C:\vcpkg_temp\installed\x64-windows-static\lib\jpeg.lib" -Destination $destPath
Copy-Item "C:\vcpkg_temp\installed\x64-windows-static\lib\zlib.lib" -Destination $destPath

# 4. Limpiar vcpkg (ya no lo necesitas)
cd ..
Remove-Item -Recurse -Force C:\vcpkg_temp
```

#### Opción 2: Descargar de otra máquina

Si ya compilaste en otra máquina Windows, copia las bibliotecas directamente:

```powershell
# Desde la máquina con las bibliotecas
$sourcePath = ".\src-tauri\vendor\libraw-sys\libs\windows\x64"
Compress-Archive -Path $sourcePath -DestinationPath "windows-libs.zip"

# En la nueva máquina
Expand-Archive -Path "windows-libs.zip" -DestinationPath ".\src-tauri\vendor\libraw-sys\libs\windows"
```

---

## 🎯 Verificación

Después de copiar, verifica que las bibliotecas están en su lugar:

```powershell
Get-ChildItem "src-tauri\vendor\libraw-sys\libs\windows\x64"

# Deberías ver:
# raw_r.lib   (~2-5 MB)
# lcms2.lib   (~500 KB)
# jpeg.lib    (~1 MB)
# zlib.lib    (~200 KB)
```

---

## 📊 Comparación: macOS vs Windows

| Aspecto          | macOS                      | Windows (Ahora)                    |
| ---------------- | -------------------------- | ---------------------------------- |
| **Desarrollo**   | Homebrew (dinámico)        | Bibliotecas en proyecto (estático) |
| **Producción**   | Homebrew (estático)        | Bibliotecas en proyecto (estático) |
| **Instalación**  | `brew install libraw`      | Ya incluido en el proyecto ✅      |
| **Dependencias** | Sistema (macOS + Homebrew) | Autocontenido                      |

---

## 🐛 Solución de Problemas

### Error: "Bibliotecas NO encontradas"

```
cargo:warning=❌ ERROR: Faltan bibliotecas de Windows en el proyecto
cargo:warning=  ✗ LibRaw NO encontrado (raw_r.lib)
```

**Solución:** Las bibliotecas `.lib` no están en el proyecto. Sigue los "Pasos para Desarrolladores" arriba.

### Error: "link.exe failed: exit code: 1181"

Este es el error original. Significa que falta alguna biblioteca. Verifica que **todas** las 4 bibliotecas estén presentes:

```powershell
# Verificar
$libPath = "src-tauri\vendor\libraw-sys\libs\windows\x64"
@("raw_r.lib", "lcms2.lib", "jpeg.lib", "zlib.lib") | ForEach-Object {
    $file = Join-Path $libPath $_
    if (Test-Path $file) {
        Write-Host "✓ $_ OK" -ForegroundColor Green
    } else {
        Write-Host "✗ $_ FALTA" -ForegroundColor Red
    }
}
```

### Compilación muy lenta

La primera compilación será lenta (~5-10 minutos) porque Rust debe compilar todas las dependencias. Las siguientes compilaciones serán mucho más rápidas (incremental).

Para limpiar y recompilar:

```bash
cd src-tauri
cargo clean
cd ..
bun run tauri dev
```

---

## 📝 Notas Técnicas

### ¿Por qué bibliotecas en el proyecto?

1. **Simplicidad**: Los usuarios no necesitan instalar nada externo
2. **Reproducibilidad**: Mismo entorno en todas las máquinas
3. **Portabilidad**: Funciona sin configuración adicional
4. **Control de versiones**: Versiones específicas garantizadas

### Tamaño de las bibliotecas

```
raw_r.lib:  ~2-5 MB
lcms2.lib:  ~500 KB
jpeg.lib:   ~1 MB
zlib.lib:   ~200 KB
------------------------
Total:      ~4-7 MB
```

### ¿Incluir en Git?

Tienes tres opciones:

**Opción 1: Incluir directamente (Recomendado para este caso)**

```bash
git add src-tauri/vendor/libraw-sys/libs/windows/
git commit -m "Add Windows static libraries for LibRaw"
```

- ✅ Cualquiera puede clonar y compilar inmediatamente
- ❌ Aumenta el tamaño del repositorio (~5-7 MB)

**Opción 2: Git LFS**

```bash
# Configurar Git LFS para .lib
git lfs track "*.lib"
git add .gitattributes
git add src-tauri/vendor/libraw-sys/libs/windows/
```

- ✅ No afecta el tamaño del clone inicial
- ❌ Requiere Git LFS instalado

**Opción 3: No incluir**

- Documentar en README cómo obtener las bibliotecas
- ❌ Cada desarrollador debe hacerlo manualmente

### Impacto en el binario final

- **Desarrollo (debug)**: ~50-80 MB (incluye símbolos de depuración)
- **Producción (release)**: ~10-20 MB (optimizado por el perfil en Cargo.toml)

Tu `Cargo.toml` ya tiene optimizaciones:

```toml
[profile.release]
opt-level = "z"      # Optimizar para tamaño
lto = true           # Link-time optimization
strip = true         # Sin símbolos de debug
```

---

## 🔧 Archivos Modificados

### 1. `src-tauri/vendor/libraw-sys/build.rs`

```rust
fn configure_windows(is_static: bool) {
    // Ahora busca bibliotecas en:
    // vendor/libraw-sys/libs/windows/x64/

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_path = Path::new(&manifest_dir)
        .join("libs")
        .join("windows")
        .join(arch);

    // Verifica que existan todas las bibliotecas
    // y configura el enlace estático
}
```

### 2. `src-tauri/build.rs`

```rust
fn verify_libraw_for_development() {
    // Windows siempre usa enlace estático
    if os == "windows" {
        println!("cargo:rustc-cfg=feature=\"static\"");
        verify_static_libraries();
        return;
    }
}
```

---

## ✅ Resumen

**Antes:**

```
❌ Error: linking with link.exe failed: exit code: 1181
❌ Requería vcpkg instalado en el sistema
```

**Después:**

```
✅ Bibliotecas incluidas en: src-tauri/vendor/libraw-sys/libs/windows/
✅ Proyecto completamente autocontenido
✅ Solo: git clone + bun run tauri dev
```

---

## 📚 Referencias

- [LibRaw Official Site](https://www.libraw.org/)
- [vcpkg Package Manager](https://vcpkg.io/)
- [Tauri Build Guide](https://tauri.app/v1/guides/building/)
- [Rust FFI Book](https://doc.rust-lang.org/nomicon/ffi.html)

---

## 💡 Para Otros Desarrolladores

Si eres un nuevo desarrollador del proyecto:

1. **Clona el repositorio**

   ```bash
   git clone <repo-url>
   cd transform_images
   ```

2. **Si las bibliotecas están en Git** (recomendado)

   ```bash
   # Ya están incluidas, solo compila
   bun install
   bun run tauri dev
   ```

3. **Si las bibliotecas NO están en Git**
   - Lee la sección "Pasos para Desarrolladores"
   - Obtén las bibliotecas con vcpkg
   - Cópialas al proyecto
   - Luego compila

¡Y eso es todo! 🚀
