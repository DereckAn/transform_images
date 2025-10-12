# GitHub Actions Workflows

## 📦 Build and Release

El workflow `build-release.yml` automatiza la compilación y distribución de Transform Images para múltiples plataformas.

### ¿Cuándo se ejecuta?

- **Push a `main`**: Compila y testea automáticamente
- **Tags `v*`** (ej: `v1.0.0`): Crea un release con binarios
- **Pull Requests**: Verifica que los cambios compilan y pasan tests
- **Manual**: Desde la pestaña Actions en GitHub

### Plataformas Soportadas

| Plataforma | Target | Instalador |
|------------|--------|------------|
| macOS Intel | x86_64-apple-darwin | ✅ .dmg |
| macOS Apple Silicon | aarch64-apple-darwin | ✅ .dmg |
| Windows | x86_64-pc-windows-msvc | ✅ .msi/.exe |
| Linux | x86_64-unknown-linux-gnu | ✅ .AppImage/.deb |

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
- Soporta 4 plataformas en paralelo (macOS Intel/ARM, Windows, Linux)

### Estrategia de Enlace

El proyecto usa **enlace condicional** controlado por Cargo features:

- **macOS y Windows**: Enlace estático (`--features static`)
  - Binarios autocontenidos, no requieren dependencias externas
  - Mayor tamaño pero máxima portabilidad

- **Linux**: Enlace dinámico (sin features)
  - Binarios más pequeños
  - Requiere que el usuario tenga `libraw`, `liblcms2` y `libjpeg` instalados
  - Compatible con gestores de paquetes estándar (apt, dnf, pacman)

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
- ✅ Configurado para enlace dinámico (bibliotecas disponibles en repositorios)
- ℹ️ Los usuarios necesitan tener libraw instalado en su sistema
- 💡 Enlace estático posible pero requiere compilar librerías desde fuente

#### Windows
```powershell
# vcpkg se instala automáticamente en el workflow
vcpkg install libraw:x64-windows-static
vcpkg install lcms:x64-windows-static
vcpkg install libjpeg-turbo:x64-windows-static
vcpkg install zlib:x64-windows-static
```
- ✅ Configurado para enlace estático con vcpkg
- ✅ Instalación automática en GitHub Actions
- ✅ Binario autocontenido sin dependencias externas
- ℹ️ zlib es necesario para descompresión de archivos DNG en LibRaw

### Caché

El workflow usa caché de GitHub Actions para:
- Cargo registry (`~/.cargo/registry`)
- Cargo git (`~/.cargo/git`)
- Build artifacts (`src-tauri/target`)
- Bun usa caché global automática (más rápido que npm)

Esto reduce el tiempo de compilación de ~10 minutos a ~3 minutos.

### Por qué Bun en lugar de Node.js

El proyecto usa **Bun** como runtime de JavaScript por varias razones:

- ⚡ **10x más rápido** que npm para instalar dependencias
- 🚀 **Builds más rápidos** en CI/CD
- 📦 **Compatible** con el ecosistema npm (drop-in replacement)
- 🎯 **Mejor experiencia** de desarrollo local
- 💾 **Caché global** reduce espacio en disco

### Crear un Release

1. **Asegúrate de que todo funciona**:
   ```bash
   cargo test --release
   bun run tauri build
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

Los instaladores generados están disponibles en:
- **Tags de versión** (`v*`): Automáticamente adjuntos al draft release
- **Push a main/PRs**: Disponibles como artifacts temporales (90 días)

Formatos generados:
- macOS: `.dmg` y `.app.tar.gz`
- Windows: `.msi` y `.exe`
- Linux: `.AppImage` y `.deb`

Acceso: GitHub → Actions → Run específico → Artifacts (o Release si es un tag)

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

- [x] Configurar LibRaw para Windows
- [x] Sistema de enlace condicional (estático/dinámico)
- [ ] Agregar caché para vcpkg en Windows (acelerar builds)
- [ ] Considerar enlace estático en Linux (requiere compilar librerías)
- [ ] Agregar firma de código para macOS/Windows
- [ ] Agregar notarización para macOS
- [ ] Publicar automáticamente en Homebrew
- [ ] Agregar benchmarks en CI
- [ ] Agregar coverage reports
