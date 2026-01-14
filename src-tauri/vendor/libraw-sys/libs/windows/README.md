# Windows Static Libraries for LibRaw

This folder contains pre-compiled static libraries required to build the project on Windows.

## Libraries Included

| Library          | File            | Size    | Purpose                                    |
| ---------------- | --------------- | ------- | ------------------------------------------ |
| **LibRaw**       | `x64/raw_r.lib` | ~27 MB  | RAW image processing (thread-safe version) |
| **Little CMS 2** | `x64/lcms2.lib` | ~3 MB   | Color management system                    |
| **JPEG Turbo**   | `x64/jpeg.lib`  | ~5 MB   | Fast JPEG codec                            |
| **zlib**         | `x64/zlib.lib`  | ~0.4 MB | Compression library                        |

**Total:** ~36 MB

## Why Static Libraries?

The project uses **static linking** on Windows to create a self-contained binary that doesn't require external DLLs. This makes distribution much simpler for end users.

## For Developers

### If libraries are already in this folder

Just compile normally:

```bash
bun run tauri dev
```

### If libraries are missing

Run the setup script (one time only):

```powershell
.\scripts\setup-windows-libs.ps1
```

This script will:

1. Download and install vcpkg temporarily
2. Compile the required libraries
3. Copy them to this folder
4. Clean up temporary files

## Source & Licensing

These libraries are compiled from open-source projects:

- **LibRaw**: LGPL-2.1 / CDDL-1.0 - https://www.libraw.org/
- **Little CMS 2**: MIT - https://www.littlecms.com/
- **JPEG Turbo**: BSD-3-Clause - https://libjpeg-turbo.org/
- **zlib**: Zlib - https://zlib.net/

See each project's license for full details.

## Architecture Support

Currently includes libraries for:

- ✅ **x64** (64-bit Intel/AMD)

ARM64 support can be added by compiling with `arm64-windows-static` triplet.

## Build Information

Libraries compiled with:

- **vcpkg** package manager
- **MSVC** compiler (Visual Studio 2022)
- **Static runtime** linking
- **Release** configuration

## Technical Details

### Linking Configuration

The `build.rs` script automatically:

1. Detects Windows platform
2. Searches for libraries in this folder
3. Links statically with required system libraries:
   - `ws2_32.lib` (Windows Sockets)
   - `userenv.lib` (User Environment)

### Comparison with macOS

| Platform    | Development        | Production        | Source  |
| ----------- | ------------------ | ----------------- | ------- |
| **macOS**   | Dynamic (Homebrew) | Static (Homebrew) | System  |
| **Windows** | Static (Vendored)  | Static (Vendored) | Project |

Windows always uses static linking to avoid DLL distribution issues.

## Troubleshooting

### Error: "LibRaw libraries not found"

The `build.rs` script will show:

```
❌ ERROR: Faltan bibliotecas de Windows en el proyecto
```

**Solution:** Run `.\scripts\setup-windows-libs.ps1` to download libraries.

### Error: "library is invalid or corrupted"

Libraries may be corrupted. Delete this folder and run the setup script again.

### Compilation is very slow

First compilation takes ~5-10 minutes to build all dependencies. Subsequent compilations are much faster (incremental).

## Maintenance

### Updating Libraries

To update to newer versions:

1. Delete this folder
2. Run setup script to get latest from vcpkg
3. Test compilation
4. Commit updated libraries

### Verification

Check libraries are present:

```powershell
Get-ChildItem x64\*.lib
```

Should show all 4 `.lib` files.

---

**Note:** These binaries are included in the repository for convenience. If you prefer not to store binaries in Git, use Git LFS or document the setup process in your main README.
