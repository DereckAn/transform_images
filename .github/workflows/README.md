# GitHub Actions Workflows

## 📦 Build and Release

El workflow `build-release.yml` automatiza la compilación y distribución de Transform Images para múltiples plataformas.

### ¿Cuándo se ejecuta?

- **Push a `main`**: Compila y testea automáticamente
- **Tags `v*`** (ej: `v1.0.0`): Crea un release con binarios
- **Pull Requests**: Verifica que los cambios compilan y pasan tests
- **Manual**: Desde la pestaña Actions en GitHub

### Plataformas Soportadas

| Plataforma | Target | Binario Tauri | CLI Binary |
|------------|--------|---------------|------------|
| macOS Intel | x86_64-apple-darwin | ✅ .dmg | ✅ .tar.gz |
| macOS Apple Silicon | aarch64-apple-darwin | ✅ .dmg | ✅ .tar.gz |
| Windows | x86_64-pc-windows-msvc | ⚠️ .msi/.exe | ⚠️ .zip |
| Linux | x86_64-unknown-linux-gnu | ✅ .AppImage/.deb | ✅ .tar.gz |

⚠️ = Configuración de LibRaw pendiente

### Jobs del Workflow

#### 1. **Test Suite** (`test`)
- Ejecuta todos los unit tests
- Usa caché para optimizar velocidad
- Debe pasar antes de compilar

#### 2. **Build Tauri App** (`build-tauri`)
- Compila la aplicación completa (frontend + backend)
- Crea instaladores nativos (.dmg, .msi, .deb, .AppImage)
- Usa `tauri-action` oficial
- Crea draft release automáticamente en tags

#### 3. **Build CLI Binary** (`build-cli`)
- Compila solo el binario de Rust
- Útil para distribución standalone
- Genera artifacts comprimidos

### Dependencias por Plataforma

#### macOS
```bash
brew install libraw little-cms2 jpeg-turbo
```
- ✅ Configurado para enlace estático
- ✅ Soporta Intel y Apple Silicon

#### Linux
```bash
sudo apt-get install -y \
  libraw-dev \
  liblcms2-dev \
  libjpeg-dev \
  libwebkit2gtk-4.1-dev \
  libayatana-appindicator3-dev
```
- ✅ Configurado para enlace dinámico
- ⚠️ Enlace estático requiere compilar librerías

#### Windows
- ⚠️ **Pendiente**: Configurar LibRaw
- Opciones:
  - Compilar LibRaw desde fuente
  - Usar vcpkg
  - Empaquetar DLLs con el instalador

### Caché

El workflow usa caché de GitHub Actions para:
- Cargo registry (`~/.cargo/registry`)
- Cargo git (`~/.cargo/git`)
- Build artifacts (`src-tauri/target`)
- npm packages (`node_modules`)

Esto reduce el tiempo de compilación de ~10 minutos a ~3 minutos.

### Crear un Release

1. **Asegúrate de que todo funciona**:
   ```bash
   cargo test --release
   npm run tauri build
   ```

2. **Crea un tag**:
   ```bash
   git tag -a v1.0.0 -m "Release v1.0.0"
   git push origin v1.0.0
   ```

3. **GitHub Actions automáticamente**:
   - ✅ Ejecuta tests
   - ✅ Compila para todas las plataformas
   - ✅ Crea draft release
   - ✅ Sube binarios como assets

4. **Edita el draft release**:
   - Revisa las release notes
   - Agrega capturas de pantalla
   - Publica el release

### Artifacts

Los binarios compilados están disponibles como artifacts en cada run:
- `transform-images-macos-intel.tar.gz`
- `transform-images-macos-apple-silicon.tar.gz`
- `transform-images-linux.tar.gz`
- `transform-images-windows.exe.zip`

Puedes descargarlos desde la pestaña Actions → Run específico → Artifacts.

### Troubleshooting

#### Error: LibRaw not found
- **macOS**: Verifica que Homebrew esté instalado
- **Linux**: Instala `libraw-dev` con apt
- **Windows**: Pendiente de configuración

#### Error: Tests failing
- Ejecuta los tests localmente primero: `cargo test`
- Verifica que todos los cambios estén committed

#### Build muy lento
- El primer build toma ~10 minutos (descarga dependencias)
- Builds subsecuentes con caché: ~3 minutos
- Si el caché está corrupto, elimínalo desde Settings → Actions → Caches

### Mejoras Futuras

- [ ] Configurar LibRaw para Windows
- [ ] Habilitar enlace estático en Linux
- [ ] Agregar firma de código para macOS/Windows
- [ ] Agregar notarización para macOS
- [ ] Publicar automáticamente en Homebrew
- [ ] Agregar benchmarks en CI
- [ ] Agregar coverage reports
