# egui-ofl-fonts User Guide (for LLM / automation workflows)

This guide explains how to use this crate from an application project, and what happens internally during build.

## 1) What this crate does

At compile time, this crate resolves required OFL font files and embeds them into your binary.

At runtime, your app calls helper functions/macros to register those embedded fonts into `egui`.

## 2) Default behavior (when you just add the dependency)

Default features are:

- `license-ofl`
- `source-google-fonts`
- `source-local-fallback`
- `font-kiwi-maru`
- `font-hachi-maru-pop`

Effect:

- Target families are `Kiwi Maru` + `Hachi Maru Pop`
- Build tries local fallback files first (if present)
- If missing, build fetches from Google Fonts (GitHub API + raw download)
- Resolved files are written to Cargo `OUT_DIR`
- Generated registry is emitted to `OUT_DIR/generated_fonts.rs`
- Rust code includes those bytes via `include_bytes!`

## 3) Internal decision flow (what happens when)

For each required asset (`.ttf/.otf` and `OFL.txt`):

1. If `source-submodule` is enabled:
   - check `GOOGLE_FONTS_REPO_DIR` first
   - otherwise auto-clone `https://github.com/google/fonts` into `target/egui-ofl-fonts-cache/google-fonts`
2. If `source-local-fallback` is enabled:
   - check crate-local `fonts/` and `licenses/`
3. If `source-google-fonts` is enabled:
   - call GitHub contents API to list family files
   - download missing files from `raw.githubusercontent.com`
4. If all enabled sources fail:
   - build fails

## 4) Runtime API usage patterns

### Install all embedded fonts in default order

```rust
egui_ofl_fonts::install_fonts(&cc.egui_ctx);
```

### Install selected fonts by name

```rust
egui_ofl_fonts::install_selected_fonts!(
    &cc.egui_ctx,
    "KiwiMaru-Regular",
    "HachiMaruPop-Regular",
)?;
```

### Install selected fonts by enum (typed)

```rust
use egui_ofl_fonts::BuiltinFont;

egui_ofl_fonts::install_builtin_fonts!(
    &cc.egui_ctx,
    BuiltinFont::KiwiMaruRegular,
    BuiltinFont::HachiMaruPopRegular,
)?;
```

### Inspect embedded font names

```rust
let names = egui_ofl_fonts::embedded_font_names();
```

### Inspect build-time acquisition summary at runtime

```rust
let s = egui_ofl_fonts::build_acquisition_summary();
println!("total={}, cache={}, submodule={}, local={}, api={}", s.total, s.cache, s.submodule, s.local_fallback, s.api);
```

## 5) Recommended command recipes

### Default mode (API + local fallback, kiwi/hachi)

```bash
cargo clean
cargo check -vv
```

### Clone-source-only mode (`source-submodule` feature)

```bash
cargo clean
cargo check --no-default-features --features license-ofl,source-submodule,font-all -vv
```

### API-only mode

```bash
cargo clean
cargo check --no-default-features --features license-ofl,source-google-fonts,font-kiwi-maru,font-hachi-maru-pop -vv
```

### Add extra OFL families dynamically

```bash
EGUI_OFL_EXTRA_OFL_DIRS="rocknrollone,zenmarugothic" cargo check -vv
```

## 6) 403 troubleshooting (GitHub API limit / access)

If build fails with `403` while using `source-google-fonts`:

- Set `GITHUB_TOKEN` and retry:

```bash
GITHUB_TOKEN=your_token_here cargo check
```

- Or switch to clone source:

```bash
cargo check --no-default-features --features license-ofl,source-submodule,font-all
```

- Or keep API + enable local fallback files so build can proceed without network for known files.

Build script now emits explicit 403 guidance such as:

- `GitHub API returned 403 ... set GITHUB_TOKEN or use source-submodule feature`

## 7) Observability

Build script prints this summary warning:

- `egui-ofl-fonts assets: total=..., cache=..., submodule=..., local-fallback=..., api=...`

Use this to confirm which source path was actually used.

## 8) Example programs

- `cargo run --example install_default`
- `cargo run --example install_selected`
- `cargo run --example install_builtin`
- `cargo run --example font_picker_gui`

`font_picker_gui` shows embedded font list, lets you pick a font, and shows acquisition summary.
