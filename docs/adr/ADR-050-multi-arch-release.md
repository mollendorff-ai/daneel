# ADR-050: Multi-Architecture Release with MUSL and Green Coding

## Status

ACCEPTED

## Date

2025-12-29

## Authors

- Rex (Louis C. Tavares)
- Claude Opus 4.5

## Context

DANEEL needs to be distributed as pre-built binaries for multiple platforms. Users shouldn't need Rust installed to run Timmy. The build and distribution strategy should align with green coding principles - minimizing environmental impact through efficient resource usage.

### Current State

- Makefile builds `x86_64-unknown-linux-musl` for Linux (static)
- macOS builds are dynamically linked
- No automated release workflow
- No binary compression
- No multi-architecture support

### Requirements

1. Support 5 target platforms (Linux x64/ARM64, macOS x64/ARM64, Windows x64)
2. Minimize binary size (bandwidth, storage, download time)
3. Maximize portability (no runtime dependencies on Linux)
4. Minimize CI resource usage (green coding)
5. Automated releases via GitHub Actions

## Decision

### 1. MUSL for Linux (Static Linking)

**Target:** `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl`

**Rationale:**
- **Zero runtime dependencies** - Works on ANY Linux distribution (Alpine, Debian, RHEL, NixOS, etc.)
- **No GLIBC version hell** - Eliminates "GLIBC_2.XX not found" errors
- **Smaller attack surface** - No shared library injection vectors
- **Green coding** - No dynamic linker overhead at startup

**Trade-off:** MUSL binaries are slightly larger before compression, but UPX more than compensates.

### 2. UPX Compression (Linux + Windows)

**Tool:** UPX with `--best --lzma` flags

**Platforms:**
- Linux x64/ARM64: YES (works perfectly with static musl binaries)
- Windows x64: YES (works with MSVC binaries)
- macOS: NO (breaks Gatekeeper code signing)

**Expected compression:** 40-60% size reduction

**Green coding impact:**
- Smaller downloads = less bandwidth = less energy
- Smaller storage footprint = less disk I/O
- Faster CI artifact uploads = less runner time

### 3. Native Builds (No Docker)

**Strategy:** Use native GitHub Actions runners with musl-tools package

| Platform | Runner | Toolchain |
|----------|--------|-----------|
| Linux x64 musl | `ubuntu-latest` | `apt: musl-tools` |
| Linux ARM64 musl | `ubuntu-24.04-arm` | `apt: musl-tools` |
| macOS ARM | `macos-14` | native |
| macOS Intel | `macos-13` | native |
| Windows x64 | `windows-latest` | MSVC |

**Why not Docker/cross-rs:**
- **Simpler** - No container orchestration, no image pulls
- **Faster** - No Docker layer caching, no image download
- **Greener** - Less CI compute time, no container overhead
- **Transparent** - Easy to debug, standard toolchain

**Trade-off:** ARM64 runner costs 2x per minute, but releases are infrequent.

### 4. Release Trigger

**Trigger:** Git tag push (`v*`)

```bash
git tag v0.8.1
git push origin v0.8.1
```

**Why tag-based (not Cargo.toml change):**
- Explicit intent - pushing a tag is a deliberate release action
- Works with pre-release tags (`v0.8.1-rc1`)
- No accidental releases from version bumps during development

### 5. Artifact Structure

```
daneel-x86_64-unknown-linux-musl.tar.gz    # Linux x64 (static, compressed)
daneel-aarch64-unknown-linux-musl.tar.gz   # Linux ARM64 (static, compressed)
daneel-aarch64-apple-darwin.tar.gz         # macOS ARM
daneel-x86_64-apple-darwin.tar.gz          # macOS Intel
daneel-x86_64-pc-windows-msvc.zip          # Windows x64 (compressed)
SHA256SUMS.txt                              # Checksums for verification
```

## Green Coding Justification

### 1. Binary Size Reduction

| Metric | Before | After | Savings |
|--------|--------|-------|---------|
| Linux binary | ~25 MB | ~10 MB | 60% |
| Download bandwidth | 25 MB × N users | 10 MB × N users | 60% |
| GitHub storage | 125 MB (5 platforms) | ~55 MB | 56% |

### 2. CI Efficiency

| Metric | Docker/cross-rs | Native musl | Savings |
|--------|-----------------|-------------|---------|
| Docker pull | ~500 MB | 0 | 100% |
| Build overhead | Container startup | None | ~30s/build |
| Cache efficiency | Layer-based | Cargo native | Better |

### 3. Runtime Efficiency

| Metric | Dynamic (glibc) | Static (musl) | Impact |
|--------|-----------------|---------------|--------|
| Startup time | Dynamic linking | Instant | Faster |
| Memory | Shared libs loaded | Self-contained | Predictable |
| Dependencies | glibc version required | None | Portable |

### 4. Distribution Efficiency

- **Smaller binaries** = less CDN bandwidth = less energy
- **Static linking** = no dependency resolution = faster installs
- **Checksums** = verify integrity without re-downloading

## Consequences

### Positive

1. **Universal Linux compatibility** - Single binary works everywhere
2. **Smaller downloads** - 40-60% bandwidth reduction
3. **Simpler CI** - No Docker complexity
4. **Faster releases** - Native builds are faster than cross-compilation
5. **Green footprint** - Less compute, less storage, less bandwidth

### Negative

1. **ARM64 runner cost** - 2x per-minute cost (mitigated by infrequent releases)
2. **macOS uncompressed** - Can't use UPX due to code signing
3. **Slightly larger base binary** - MUSL adds ~1-2 MB before compression

### Neutral

1. **No Windows ARM64** - Low demand, can add later if needed
2. **No 32-bit support** - Legacy, not worth the CI cost

## Implementation

### Files Created

- `.github/workflows/release.yml` - Release workflow
- `docs/adr/ADR-050-multi-arch-release.md` - This document

### Release Process

```bash
# 1. Update version in Cargo.toml
# 2. Update CHANGELOG.md
# 3. Commit changes
git add -A && git commit -m "v0.8.1: <description>"

# 4. Create and push tag
git tag v0.8.1
git push origin main
git push origin v0.8.1

# 5. GitHub Actions automatically:
#    - Runs tests
#    - Builds 5 platforms
#    - Compresses with UPX (Linux + Windows)
#    - Creates GitHub Release with artifacts
```

### Verification

```bash
# Verify static linking (Linux)
ldd daneel
# Output: "not a dynamic executable"

# Verify UPX compression
file daneel
# Output: "ELF 64-bit ... (UPX compressed)"

# Verify checksum
sha256sum -c SHA256SUMS.txt
```

## References

- [cross-rs](https://github.com/cross-rs/cross) - Evaluated but rejected for simplicity
- [UPX](https://upx.github.io/) - Binary compression
- [musl libc](https://musl.libc.org/) - Lightweight libc implementation
- [Green Software Foundation](https://greensoftware.foundation/) - Green coding principles
- `../forge-demo/.github/workflows/release.yml` - Reference implementation
- `../asimov/.github/workflows/release.yml` - Reference implementation

## Appendix: UPX Compatibility Matrix

| Platform | UPX Support | Notes |
|----------|-------------|-------|
| Linux x64 glibc | YES | Works |
| Linux x64 musl | YES | Works great (static) |
| Linux ARM64 glibc | YES | Works |
| Linux ARM64 musl | YES | Works great (static) |
| macOS x64 | NO | Breaks code signing |
| macOS ARM | NO | Breaks code signing |
| Windows x64 | YES | Works |
| Windows ARM64 | NO | Not supported by UPX |
