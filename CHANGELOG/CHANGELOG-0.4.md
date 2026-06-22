# Change Log

All notable changes to the "shiroa" will be documented in this file.

Check [Keep a Changelog](http://keepachangelog.com/) for recommendations on how to structure this file.

## v0.4.0 - [2026-06-22]

- Bumped shiroa CLI/frontend version to v0.4.0 in this release series.
- Bumped Typst and Tinymist dependencies to v0.15.0 in https://github.com/Myriad-Dreamin/shiroa/pull/244
- Bumped typst.ts to v0.7.0-rc2 in https://github.com/Myriad-Dreamin/shiroa/pull/227
- Bumped Typst to v0.14.2 by @lovesegfault in https://github.com/Myriad-Dreamin/shiroa/pull/238
- Migrated frontend tooling from Yarn to pnpm in https://github.com/Myriad-Dreamin/shiroa/pull/245
- Updated cargo-dist release workflow and cache configuration in https://github.com/Myriad-Dreamin/shiroa/pull/248 and https://github.com/Myriad-Dreamin/shiroa/pull/249

### CLI

- Added `--package-path` and `TYPST_PACKAGE_PATH` support for local Typst package sources by @omega-800 in https://github.com/Myriad-Dreamin/shiroa/pull/236
- Added `--input KEY=VALUE` passthrough to `sys.inputs` by @lovesegfault in https://github.com/Myriad-Dreamin/shiroa/pull/240
- Deprecated the `-w` flag in https://github.com/Myriad-Dreamin/shiroa/pull/215
- Added and tested a minimal project for the `shiroa init` flow in https://github.com/Myriad-Dreamin/shiroa/pull/214
- Fixed CLI argument names in documentation by @ice1000 and @xiyihan0 in https://github.com/Myriad-Dreamin/shiroa/pull/226 and https://github.com/Myriad-Dreamin/shiroa/pull/220

### Theme

- Introduced the starlight theme and rewrote the mdbook theme in pure Typst in https://github.com/Myriad-Dreamin/shiroa/pull/157 and https://github.com/Myriad-Dreamin/shiroa/pull/168
- Added `partbreak` by @NiklasEi in https://github.com/Myriad-Dreamin/shiroa/pull/235
- (Fix) Avoided sidebar crashes for empty parts by @lovesegfault in https://github.com/Myriad-Dreamin/shiroa/pull/239
- (Fix) Improved inline math baseline alignment for static HTML mode by @JackyJnirvana in https://github.com/Myriad-Dreamin/shiroa/pull/234
- (Fix) Used the Typst HTML std key for HTML target detection in https://github.com/Myriad-Dreamin/shiroa/pull/246
- Added divider support in table of contents in https://github.com/Myriad-Dreamin/shiroa/pull/188
- Improved link handling, URL-base usage, metadata, and description/search behavior in https://github.com/Myriad-Dreamin/shiroa/pull/164, https://github.com/Myriad-Dreamin/shiroa/pull/166, https://github.com/Myriad-Dreamin/shiroa/pull/185, https://github.com/Myriad-Dreamin/shiroa/pull/192, https://github.com/Myriad-Dreamin/shiroa/pull/195, https://github.com/Myriad-Dreamin/shiroa/pull/199, https://github.com/Myriad-Dreamin/shiroa/pull/205, https://github.com/Myriad-Dreamin/shiroa/pull/209, and https://github.com/Myriad-Dreamin/shiroa/pull/210

### Package

- (Fix) Corrected show rules, plain-text behavior, and inline math/package rendering issues in https://github.com/Myriad-Dreamin/shiroa/pull/120, https://github.com/Myriad-Dreamin/shiroa/pull/122, https://github.com/Myriad-Dreamin/shiroa/pull/133, https://github.com/Myriad-Dreamin/shiroa/pull/134, https://github.com/Myriad-Dreamin/shiroa/pull/138, and https://github.com/Myriad-Dreamin/shiroa/pull/234
- (Fix) Corrected outer-height value in xcommand-html by @minimarimo3 in https://github.com/Myriad-Dreamin/shiroa/pull/171
- (Fix) Resolved multi-byte character issues and related description bugs by @minimarimo3 in https://github.com/Myriad-Dreamin/shiroa/pull/173 and https://github.com/Myriad-Dreamin/shiroa/pull/177
- Set up Typst package tests in https://github.com/Myriad-Dreamin/shiroa/pull/121
- Published Typst packages by workflow and added existing-package detection in https://github.com/Myriad-Dreamin/shiroa/pull/200, https://github.com/Myriad-Dreamin/shiroa/pull/201, and https://github.com/Myriad-Dreamin/shiroa/pull/202

### Rendering

- Switched HTML export on by default and supported cross links in static HTML pages in https://github.com/Myriad-Dreamin/shiroa/pull/130 and https://github.com/Myriad-Dreamin/shiroa/pull/132
- Added HTML renderer metadata support and refactored rendering logic by @QuadnucYard in https://github.com/Myriad-Dreamin/shiroa/pull/155
- Parallelized compilation, added watch compilation for serving, and improved compilation output in https://github.com/Myriad-Dreamin/shiroa/pull/124, https://github.com/Myriad-Dreamin/shiroa/pull/126, and https://github.com/Myriad-Dreamin/shiroa/pull/127
- Ignored the foreign object warning during compilation in https://github.com/Myriad-Dreamin/shiroa/pull/125
- Used TextExport that respects style tags in https://github.com/Myriad-Dreamin/shiroa/pull/154
- Fixed the initialized `page.typ` book path by @Sinofine in https://github.com/Myriad-Dreamin/shiroa/pull/174

### Docs

- Updated README and installation documentation by @HoiGe in https://github.com/Myriad-Dreamin/shiroa/pull/119
- Updated introduction, FAQ, getting started, installation, metadata, theme, support, and todo documentation in https://github.com/Myriad-Dreamin/shiroa/pull/137, https://github.com/Myriad-Dreamin/shiroa/pull/194, https://github.com/Myriad-Dreamin/shiroa/pull/206, https://github.com/Myriad-Dreamin/shiroa/pull/213, and https://github.com/Myriad-Dreamin/shiroa/pull/218
- (Docs) Fixed `book.toml` to `book.typ` typo by @ttlns in https://github.com/Myriad-Dreamin/shiroa/pull/204
- (Docs) Replaced an en-dash with two hyphens for long flag spelling in https://github.com/Myriad-Dreamin/shiroa/pull/208

### Misc

- (Fix) Ensured the wasm module matches the frontend build by @kxxt in https://github.com/Myriad-Dreamin/shiroa/pull/101
- (Fix) Pulled the fix for CVE-2025-55159 in https://github.com/Myriad-Dreamin/shiroa/pull/182
- Refactored modules and split files by @QuadnucYard in https://github.com/Myriad-Dreamin/shiroa/pull/229
- Updated release runner configuration from tinymist in https://github.com/Myriad-Dreamin/shiroa/pull/228

**Full Changelog**: https://github.com/Myriad-Dreamin/shiroa/compare/v0.3.0...v0.4.0
