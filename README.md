# egui-ofl-fonts

Small helper crate for loading bundled OFL fonts into `egui`.

This crate bundles only two OFL font families and provides helpers to register them in `egui`.

## Bundled fonts

- `HachiMaruPop-Regular` (from Hachi Maru Pop project)
- `KiwiMaru-Regular`, `KiwiMaru-Medium`, `KiwiMaru-Light` (from Kiwi Maru project)

## Upstream distribution (font sources)

The bundled `.ttf` files are sourced from the following upstream distributions:

- **Hachi Maru Pop**
    - Google Fonts specimen: <https://fonts.google.com/specimen/Hachi+Maru+Pop>
    - Google Fonts repository source: <https://github.com/google/fonts/tree/main/ofl/hachimarupop>
- **Kiwi Maru**
    - Google Fonts specimen: <https://fonts.google.com/specimen/Kiwi+Maru>
    - Google Fonts repository source: <https://github.com/google/fonts/tree/main/ofl/kiwimaru>

Only these 2 OFL font families are bundled in this crate.

## License

- Rust source code in this crate: MIT (see `LICENSE`)
- Bundled font files: SIL Open Font License 1.1
    - Hachi Maru Pop: `licenses/OFL-Hachi_Maru_Pop.txt`
    - Kiwi Maru: `licenses/OFL-Kiwi_Maru.txt`

When redistributing this crate (or repackaging font files), keep the corresponding OFL license texts above.

## Usage

```rust
let mut options = eframe::NativeOptions::default();
let app = move |cc: &eframe::CreationContext<'_>| {
    egui_ofl_fonts::install_fonts(&cc.egui_ctx);
    Ok(Box::new(MyApp::default()))
};
```
