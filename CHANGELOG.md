
# Change Log

## [0.2.0] - Unreleased

## [0.1.2] - Unreleased

### Added

- Android build support ([#3](https://github.com/orangethewell/open-witness-library/issues/3))
- Swiping controls on document view
- Publication styling dynamic download instead of bundled (Legal reasons)

### Deprecated

- Moved data directory from `$APPDATA_DIR/open-witness-library` to `$APPLOCALDATA_DIR/com.open-witness-library.app` for compliance with Android platform that doesn't support the past API
    - For more information, check the [Tauri Docs](https://docs.rs/tauri/2.1.1/tauri/path/struct.PathResolver.html#method.data_dir).

## [0.1.1] - 2024-11-20

### Fixed

- Fixed table of content not showing for current publication

## [0.1.0] - 2024-11-19

### Added

- MaterialUI components and visual style
- Library view with publication categories
- Category view for almost every publication type
- Settings view with language and theme switching support
- New table of contents view
- Section as tabs on TOC
- Title separators on TOC

### Changed

- Added Publication API separated from Catalog, with document cache
- Improved Catalog API with publication cache

### Deprecated

- Deprecated media-location command for get media from current open publication
- Deprecated old `PubManager` for using new Catalog API
- Deprecated `extension.rs` module

### Removed

- Removed old handwritten publication style css file

### Fixed

- Fix #1 by adding a UI for installing publications
- Fix #6 for installing publication process
