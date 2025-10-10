# Plan de Desarrollo - Transform Images App

## 📋 Resumen Ejecutivo

Aplicación de escritorio multiplataforma para optimización y transformación de imágenes con procesamiento multihilo, construida con Tauri 2 + TypeScript + Rust.

### Características Principales (MVP)
- ✅ Optimización de PNG/JPG sin pérdida de calidad
- ✅ Transformaciones: resize, rotate/flip, cambio de formato
- ✅ Procesamiento multihilo para velocidad
- ✅ Drag & drop de archivos y carpetas
- ✅ Control de calidad de compresión
- ✅ Selección de directorio de salida
- ✅ Preservación opcional de metadatos EXIF
- ✅ Barra de progreso en tiempo real
- ✅ Cancelación de operaciones

### Características Futuras
- 🔮 Soporte completo de formatos (WEBP, GIF, TIFF, HEIC, ICO)
- 🔮 Conversión correcta de ICO (binaria, no falsa)
- 🔮 Más transformaciones avanzadas

---

## 🏗️ Arquitectura

### Patrón Arquitectónico: Clean Architecture + Hexagonal

```
┌─────────────────────────────────────────────────────────────┐
│                         Frontend (UI)                        │
│                    TypeScript + HTML/CSS                     │
└────────────────────────────┬────────────────────────────────┘
                             │ Tauri Commands
┌────────────────────────────▼────────────────────────────────┐
│                    Application Layer                         │
│              (Coordinación, Estado, Comandos)                │
└────────────────────────────┬────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────┐
│                      Domain Layer                            │
│           (Lógica de negocio, Entidades, Casos de uso)      │
└────────────────────────────┬────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────┐
│                   Infrastructure Layer                       │
│        (Procesamiento de imágenes, Sistema de archivos)     │
└─────────────────────────────────────────────────────────────┘
```

### Principios SOLID Aplicados
- **S**ingle Responsibility: Cada módulo tiene una responsabilidad única
- **O**pen/Closed: Abierto a extensión, cerrado a modificación
- **L**iskov Substitution: Interfaces consistentes
- **I**nterface Segregation: Interfaces específicas por caso de uso
- **D**ependency Inversion: Depender de abstracciones, no implementaciones

---

## 📁 Estructura de Carpetas Propuesta

```
transform-images/
├── src/                                    # Frontend
│   ├── main.ts                            # Entry point
│   ├── app/
│   │   ├── App.ts                         # Controlador principal
│   │   ├── state/
│   │   │   └── AppState.ts               # Estado global
│   │   └── services/
│   │       ├── ImageService.ts           # Comunicación con backend
│   │       └── FileService.ts            # Manejo de archivos
│   ├── components/                        # Componentes UI
│   │   ├── DropZone/
│   │   ├── ImageList/
│   │   ├── ProgressBar/
│   │   ├── SettingsPanel/
│   │   └── OutputSelector/
│   ├── models/                            # Tipos TypeScript
│   │   └── types.ts
│   └── utils/
│       └── helpers.ts
│
├── src-tauri/                             # Backend
│   ├── src/
│   │   ├── main.rs                       # Entry point
│   │   ├── lib.rs                        # Tauri app setup
│   │   │
│   │   ├── application/                  # Application Layer
│   │   │   ├── mod.rs
│   │   │   ├── commands.rs              # Tauri commands
│   │   │   ├── state.rs                 # Estado compartido
│   │   │   └── dto.rs                   # Data Transfer Objects
│   │   │
│   │   ├── domain/                       # Domain Layer
│   │   │   ├── mod.rs
│   │   │   ├── models/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── image.rs            # Entidad Image
│   │   │   │   ├── transformation.rs   # Transformaciones
│   │   │   │   └── settings.rs         # Configuración
│   │   │   ├── services/
│   │   │   │   ├── mod.rs
│   │   │   │   └── processor.rs        # Interfaz del procesador
│   │   │   └── value_objects/
│   │   │       ├── mod.rs
│   │   │       ├── image_format.rs
│   │   │       ├── dimensions.rs
│   │   │       └── quality.rs
│   │   │
│   │   └── infrastructure/               # Infrastructure Layer
│   │       ├── mod.rs
│   │       ├── image_processor/
│   │       │   ├── mod.rs
│   │       │   ├── processor_impl.rs   # Implementación
│   │       │   ├── optimizers/
│   │       │   │   ├── mod.rs
│   │       │   │   ├── png_optimizer.rs
│   │       │   │   └── jpg_optimizer.rs
│   │       │   └── transformers/
│   │       │       ├── mod.rs
│   │       │       ├── resizer.rs
│   │       │       └── rotator.rs
│   │       └── file_system/
│   │           ├── mod.rs
│   │           └── file_handler.rs
│   │
│   └── Cargo.toml
│
└── docs/
    ├── architecture.md
    └── patterns.md
```

---

## 🛠️ Stack Tecnológico

### Frontend
- **TypeScript**: Tipado estático
- **Vite**: Build tool rápido
- **CSS Modules** o **TailwindCSS**: Para estilos

### Backend (Rust)
| Librería | Propósito | Versión |
|----------|-----------|---------|
| `tauri` | Framework de aplicación | 2.x |
| `tokio` | Runtime async multihilo | 1.x |
| `image` | Manipulación de imágenes | 0.25.x |
| `oxipng` | Optimización PNG lossless | 9.x |
| `mozjpeg` | Optimización JPG superior | 0.10.x |
| `rayon` | Paralelismo data parallelism | 1.x |
| `serde` | Serialización | 1.x |
| `thiserror` | Error handling ergonómico | 1.x |
| `kamadak-exif` | Manejo de metadatos EXIF | 0.5.x |

### Futuro (Post-MVP)
- `webp`: Soporte WebP
- `libheif-rs`: Soporte HEIF/HEIC
- `ico`: Conversión ICO correcta

---

## 🎨 Patrones de Diseño

### 1. **Strategy Pattern** (Optimizadores)
Diferentes algoritmos de optimización intercambiables.

```rust
trait ImageOptimizer {
    fn optimize(&self, image: &Image, quality: Quality) -> Result<Vec<u8>>;
}

struct PngOptimizer;
struct JpgOptimizer;

impl ImageOptimizer for PngOptimizer { /* ... */ }
impl ImageOptimizer for JpgOptimizer { /* ... */ }
```

### 2. **Builder Pattern** (Configuración)
Construcción flexible de opciones de transformación.

```rust
TransformationOptions::builder()
    .resize(1920, 1080)
    .quality(85)
    .preserve_metadata(true)
    .build()
```

### 3. **Repository Pattern** (Acceso a archivos)
Abstracción del sistema de archivos.

```rust
trait ImageRepository {
    fn read(&self, path: &Path) -> Result<Image>;
    fn write(&self, path: &Path, image: &Image) -> Result<()>;
}
```

### 4. **Command Pattern** (Operaciones)
Encapsular operaciones como objetos.

```rust
trait Command {
    fn execute(&self) -> Result<()>;
    fn undo(&self) -> Result<()>;
}

struct OptimizeCommand { /* ... */ }
```

### 5. **Observer Pattern** (Progreso)
Notificar cambios de progreso a la UI.

```rust
trait ProgressObserver {
    fn on_progress(&self, current: usize, total: usize);
    fn on_complete(&self);
}
```

---

## 📅 Plan de Desarrollo en Fases

### 🎯 Fase 0: Setup (Semana 1)
**Objetivo**: Preparar el entorno y estructura básica

**Tareas**:
1. ✅ Inicializar estructura de carpetas
2. ✅ Configurar Cargo.toml con dependencias
3. ✅ Configurar tsconfig.json
4. ✅ Crear módulos básicos vacíos
5. ✅ Setup de testing

**Entregable**: Proyecto compila y corre con estructura vacía

---

### 🎯 Fase 1: Domain Layer (Semana 1-2)
**Objetivo**: Definir la lógica de negocio pura

**Tareas**:
1. Crear value objects (ImageFormat, Dimensions, Quality)
2. Crear entidad Image
3. Crear modelo Transformation
4. Definir interfaces (traits) del procesador
5. Implementar validaciones de negocio

**Entregable**: Domain layer con tests unitarios

**Código de ejemplo**:
```rust
// src-tauri/src/domain/value_objects/image_format.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Webp,
    // Más formatos después...
}

// src-tauri/src/domain/value_objects/quality.rs
#[derive(Debug, Clone, Copy)]
pub struct Quality(u8);

impl Quality {
    pub fn new(value: u8) -> Result<Self, DomainError> {
        if value > 100 {
            return Err(DomainError::InvalidQuality);
        }
        Ok(Quality(value))
    }
}

// src-tauri/src/domain/models/image.rs
pub struct Image {
    pub path: PathBuf,
    pub format: ImageFormat,
    pub dimensions: Dimensions,
    pub size_bytes: u64,
    pub metadata: Option<ImageMetadata>,
}
```

---

### 🎯 Fase 2: Infrastructure - Procesamiento Básico (Semana 2-3)
**Objetivo**: Implementar optimización PNG/JPG

**Tareas**:
1. Implementar PngOptimizer con oxipng
2. Implementar JpgOptimizer con mozjpeg
3. Implementar lectura/escritura de archivos
4. Crear factory de optimizadores
5. Testing con imágenes reales

**Entregable**: Backend procesa PNG/JPG correctamente

**Código de ejemplo**:
```rust
// src-tauri/src/infrastructure/image_processor/optimizers/png_optimizer.rs
use oxipng::{Options, optimize_from_memory};

pub struct PngOptimizer;

impl ImageOptimizer for PngOptimizer {
    fn optimize(&self, data: &[u8], quality: Quality) -> Result<Vec<u8>> {
        let opts = Options {
            compression: /* basado en quality */,
            ..Default::default()
        };

        optimize_from_memory(data, &opts)
            .map_err(|e| InfraError::OptimizationFailed(e.to_string()))
    }
}
```

---

### 🎯 Fase 3: Procesamiento Multihilo (Semana 3)
**Objetivo**: Paralelizar procesamiento de imágenes

**Tareas**:
1. Implementar pool de workers con Rayon
2. Crear sistema de colas
3. Implementar cancelación de tareas
4. Gestión de memoria eficiente
5. Testing de concurrencia

**Entregable**: Procesa múltiples imágenes en paralelo

**Código de ejemplo**:
```rust
use rayon::prelude::*;

pub struct BatchProcessor {
    max_threads: usize,
}

impl BatchProcessor {
    pub fn process_batch(
        &self,
        images: Vec<Image>,
        options: TransformationOptions,
        progress_callback: impl Fn(usize, usize) + Sync
    ) -> Result<Vec<ProcessedImage>> {
        let total = images.len();
        let counter = AtomicUsize::new(0);

        images.par_iter()
            .map(|img| {
                let result = self.process_single(img, &options)?;
                let count = counter.fetch_add(1, Ordering::SeqCst);
                progress_callback(count + 1, total);
                Ok(result)
            })
            .collect()
    }
}
```

---

### 🎯 Fase 4: Application Layer - Comandos Tauri (Semana 4)
**Objetivo**: Conectar backend con frontend

**Tareas**:
1. Crear DTOs para comunicación frontend-backend
2. Implementar comandos Tauri
3. Crear estado compartido thread-safe
4. Sistema de eventos para progreso
5. Manejo de errores consistente

**Entregable**: Frontend puede invocar comandos

**Código de ejemplo**:
```rust
// src-tauri/src/application/commands.rs
use tauri::State;

#[tauri::command]
pub async fn optimize_images(
    images: Vec<ImageDto>,
    options: OptimizationOptionsDto,
    app_state: State<'_, AppState>,
    window: tauri::Window
) -> Result<Vec<ProcessedImageDto>, String> {
    let processor = app_state.processor.lock().await;

    processor.process_batch(images, options, |current, total| {
        window.emit("progress", ProgressPayload { current, total })
            .unwrap();
    })
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cancel_operation(
    app_state: State<'_, AppState>
) -> Result<(), String> {
    app_state.cancel_signal.store(true, Ordering::SeqCst);
    Ok(())
}
```

---

### 🎯 Fase 5: Frontend - UI Básica (Semana 4-5)
**Objetivo**: Interfaz funcional drag & drop

**Tareas**:
1. Implementar zona de drag & drop
2. Lista de imágenes con información
3. Panel de configuración (calidad)
4. Selector de carpeta de salida
5. Barra de progreso

**Entregable**: UI completa y funcional

**Código de ejemplo**:
```typescript
// src/app/services/ImageService.ts
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export class ImageService {
    async optimizeImages(
        images: ImageDto[],
        options: OptimizationOptions
    ): Promise<ProcessedImage[]> {
        return invoke('optimize_images', { images, options });
    }

    onProgress(callback: (current: number, total: number) => void) {
        return listen<ProgressPayload>('progress', (event) => {
            callback(event.payload.current, event.payload.total);
        });
    }

    async cancelOperation(): Promise<void> {
        return invoke('cancel_operation');
    }
}
```

---

### 🎯 Fase 6: Transformaciones (Semana 5-6)
**Objetivo**: Resize y rotate/flip

**Tareas**:
1. Implementar resize con múltiples algoritmos
2. Implementar rotate/flip
3. Agregar controles en UI
4. Preservar aspect ratio opcional
5. Testing de transformaciones

**Entregable**: Transformaciones funcionando

**Código de ejemplo**:
```rust
// src-tauri/src/infrastructure/image_processor/transformers/resizer.rs
use image::{DynamicImage, imageops};

pub struct Resizer;

impl Resizer {
    pub fn resize(
        &self,
        img: &DynamicImage,
        width: u32,
        height: u32,
        preserve_aspect: bool,
        filter: FilterType
    ) -> DynamicImage {
        if preserve_aspect {
            img.resize(width, height, filter)
        } else {
            img.resize_exact(width, height, filter)
        }
    }
}
```

---

### 🎯 Fase 7: Metadatos EXIF (Semana 6)
**Objetivo**: Preservar/eliminar metadatos

**Tareas**:
1. Implementar lectura de EXIF
2. Implementar escritura de EXIF
3. Opción en UI para preservar metadatos
4. Mostrar información EXIF relevante
5. Testing con fotos reales

**Entregable**: Gestión de metadatos completa

---

### 🎯 Fase 8: Pulido y Testing (Semana 7)
**Objetivo**: App lista para usar

**Tareas**:
1. Testing end-to-end
2. Optimización de performance
3. Manejo de errores robusto
4. Logging y debugging
5. Documentación de usuario

**Entregable**: MVP completo y probado

---

## 🧪 Estrategia de Testing

### Tests Unitarios
- Domain layer: 100% cobertura
- Value objects con casos edge
- Validaciones de negocio

### Tests de Integración
- Procesamiento real de imágenes
- Lectura/escritura de archivos
- Optimizadores con diferentes formatos

### Tests de Performance
- Benchmark con 100, 1000, 10000 imágenes
- Uso de memoria
- Tiempo de procesamiento

### Tests E2E
- Flujo completo drag & drop → procesamiento → guardado
- Cancelación de operaciones
- Manejo de errores

---

## 📊 Métricas de Calidad

- **Cobertura de código**: >80%
- **Tiempo de compilación**: <60s
- **Procesamiento**: >10 imágenes/segundo (PNG 1MB)
- **Uso de memoria**: <500MB con 1000 imágenes
- **Reducción de tamaño**: 30-70% manteniendo calidad

---

## 🚀 Comandos de Desarrollo

```bash
# Desarrollo
bun run tauri dev

# Build
bun run tauri build

# Tests (Rust)
cd src-tauri && cargo test

# Tests con output
cargo test -- --nocapture

# Benchmark
cargo bench

# Check sin compilar
cargo check

# Clippy (linter)
cargo clippy

# Format
cargo fmt
```

---

## 📚 Recursos de Aprendizaje

### Tauri
- [Tauri Docs](https://tauri.app/v2/guides/)
- [Calling Rust from Frontend](https://tauri.app/v2/guides/features/command/)
- [Events](https://tauri.app/v2/guides/features/events/)

### Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Async Book](https://rust-lang.github.io/async-book/)

### Procesamiento de Imágenes
- [image crate docs](https://docs.rs/image/)
- [oxipng](https://github.com/shssoichiro/oxipng)
- [mozjpeg](https://github.com/mozilla/mozjpeg)

### Patrones
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)
- [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)

---

## 🎯 Próximos Pasos

1. **Revisar este plan** y hacerme preguntas
2. **Comenzar con Fase 0**: Setup del proyecto
3. **Ir fase por fase**: No pasar a la siguiente hasta completar la actual
4. **Pedir código específico**: Te daré ejemplos completos para copiar
5. **Testing continuo**: Probar cada feature antes de continuar

¿Estás listo para comenzar con la Fase 0? 🚀
