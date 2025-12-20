# Quak Images

Quak Images es una app de escritorio construida con **Tauri 2**, **TypeScript** y **Rust** para optimizar, convertir y transformar lotes de fotografías (incluyendo RAW de Sony/Canon/Nikon). El frontend corre con Vite + Tailwind, mientras que el backend aplica una arquitectura Clean/Hexagonal con `libraw`, `mozjpeg`, `oxipng` y `libwebp` para conseguir binarios rápidos y portables.

## Características destacadas
- Arrastra carpetas o archivos y obtén metadatos instantáneos (formato, dimensiones, peso).
- Conversión cruzada entre PNG, JPEG y WebP con controles fines de calidad.
- Transformaciones avanzadas: resize con filtros (Lanczos, Triangle), rotaciones y flips independientes.
- Procesamiento multihilo con cancelación, barra de progreso y estadísticas de ahorro.
- Limpieza automática de metadatos EXIF para reducir aún más el peso final.

## Instalación de binarios
Cada release publica instaladores con este patrón: `QuakImages-{versión}-{plataforma}.{ext}`.

- **macOS (Intel y Apple Silicon)**  
  1. Descarga el `.dmg` correspondiente.  
  2. Abre el DMG y arrastra Quak Images a `Applications`.  
  3. Como no tenemos Apple Developer ID, macOS lo marcará como “desarrollador no verificado”. En Finder, `Control + clic` → **Abrir** → confirma la advertencia.  
  4. Para forzar la verificación desde Terminal puedes ejecutar (copy/paste):
     ```bash
     xattr -cr "/Applications/Quak Images.app"
     ```
     Esto elimina los atributos de cuarentena y evita que Gatekeeper bloquee la app.
- **Windows 10/11**  
  - Usa el `.msi` o `-Setup.exe`. SmartScreen mostrará “Publisher: Unknown”; presiona **More info** → **Run anyway**.
- **Linux (x86_64)**  
  - `AppImage`: `chmod +x QuakImages-<versión>-Linux-x64.AppImage && ./QuakImages-<versión>-Linux-x64.AppImage`.  
  - `deb`/`rpm`: instala con tu gestor (`sudo dpkg -i ...` o `sudo rpm -i ...`).

## Requisitos previos para desarrollo
- Node 18+ (o Bun, si prefieres `bun install`).
- Rust stable + toolchain para el objetivo que necesites (`rustup target add aarch64-apple-darwin`, etc.).
- Dependencias del sistema para Tauri (GTK/webkit en Linux, XCode CLTs en macOS, Microsoft VC++ build tools en Windows). Consulta la [guía oficial](https://tauri.app/start/prerequisites/).

## Configuración y scripts útiles
```bash
# Instalar dependencias
bun install          # o npm install

# UI solamente
npm run dev          # Vite con hot reload

# App completa (TS + Rust) en modo desarrollo
npm run tauri:dev

# Build de producción + bundle Tauri
npm run build        # chequea TypeScript y genera dist/
npm run tauri:build  # empaqueta para el SO actual

# Pruebas backend
cd src-tauri && cargo test
```
Consejo: antes de empacar, ejecuta `npm run build` para asegurarte de que el frontend pasa el chequeo estricto de TypeScript (sin `any` implícitos ni imports huérfanos).

## Cómo funciona
1. `src/main.ts` registra listeners y orquesta la UI (dropzone, sliders, checkboxes).
2. Las peticiones pasan por `ImageService` → comandos Tauri (`application/commands.rs`).
3. El dominio valida formatos (`domain/value_objects`) y construye `ProcessingSettings`.
4. `ImageProcessorImpl` carga la imagen (o usa `RawProcessor` si es RAW), aplica `transformers` y después invoca al optimizador adecuado:
   - `oxipng` para PNG, `mozjpeg` para JPEG/RAW y `libwebp` para WebP con calidades ajustables.
5. `MetadataCleaner` remueve EXIF antes de escribir el archivo final.

## Pruebas y calidad
- Usa `cargo test --lib` para validar la capa de dominio e infraestructura (hay pruebas dedicadas a conversión RAW → JPEG/PNG/WebP).
- Añade pruebas de integración específicas en `src-tauri/tests/` cuando introduzcas nuevos formatos o transformaciones.
- Documenta los pasos de verificación manual (drag & drop, cancelación, preservación de metadatos) dentro de tus PRs.

## CI/CD y releases
El workflow `.github/workflows/build-release.yml` corre pruebas en Ubuntu y, tras pasar, construye binarios para **macOS Intel (macos-13)**, **macOS Apple Silicon (macos-14)**, **Windows x64** y **Linux x64**. En pushes a `main` genera artifacts; cuando el ref es un tag `v*`, publica un release en borrador con instaladores renombrados (`QuakImages-{versión}-<plataforma>`). De forma local puedes generar los mismos bundles con `npm run tauri:build` usando el target deseado.
