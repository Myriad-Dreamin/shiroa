# Change Log

All notable changes to the "shiroa" will be documented in this file.

Check [Keep a Changelog](http://keepachangelog.com/) for recommendations on how to structure this file.

## v0.3.1 - [2025-12-06]

- Bumped shiroa package to v0.3.1 in https://github.com/Myriad-Dreamin/shiroa/pull/131, https://github.com/Myriad-Dreamin/shiroa/pull/141, https://github.com/Myriad-Dreamin/shiroa/pull/221, and https://github.com/Myriad-Dreamin/shiroa/pull/223
- Bumped zebraw to v0.5.2 in https://github.com/Myriad-Dreamin/shiroa/pull/139
- Bumped typst to v0.14.0 in https://github.com/Myriad-Dreamin/shiroa/pull/219
- Bumped typst.ts to v0.6.1-rc3 in https://github.com/Myriad-Dreamin/shiroa/pull/193

### Theme

- Introduced startlight in https://github.com/Myriad-Dreamin/shiroa/pull/157, https://github.com/Myriad-Dreamin/shiroa/pull/160, https://github.com/Myriad-Dreamin/shiroa/pull/161, https://github.com/Myriad-Dreamin/shiroa/pull/162, https://github.com/Myriad-Dreamin/shiroa/pull/163, https://github.com/Myriad-Dreamin/shiroa/pull/164, and https://github.com/Myriad-Dreamin/shiroa/pull/166
- Rewrote mdbook theme to pure typst in https://github.com/Myriad-Dreamin/shiroa/pull/168, https://github.com/Myriad-Dreamin/shiroa/pull/191, and https://github.com/Myriad-Dreamin/shiroa/pull/205
- Checking and Documented `description`, `authors`, `github-link`, and `discord-link` in https://github.com/Myriad-Dreamin/shiroa/pull/206, https://github.com/Myriad-Dreamin/shiroa/pull/209, https://github.com/Myriad-Dreamin/shiroa/pull/192, and https://github.com/Myriad-Dreamin/shiroa/pull/194
- Added `x-url-base` and `x-current` in https://github.com/Myriad-Dreamin/shiroa/pull/185, https://github.com/Myriad-Dreamin/shiroa/pull/195, and https://github.com/Myriad-Dreamin/shiroa/pull/199
- Supporting divider in toc in https://github.com/Myriad-Dreamin/shiroa/pull/188

### Package

- (Fix) Corrected show rule for equations in https://github.com/Myriad-Dreamin/shiroa/pull/120
- (Fix) Strengthened `plain-text` implementation in https://github.com/Myriad-Dreamin/shiroa/pull/122 and https://github.com/Myriad-Dreamin/shiroa/pull/138
- (Fix) Corrected outer-height value in xcommand-html by @minimarimo3 in https://github.com/Myriad-Dreamin/shiroa/pull/171
- (Fix) Resolved multi-byte character issue in auto description generation by @minimarimo3 in https://github.com/Myriad-Dreamin/shiroa/pull/173 and https://github.com/Myriad-Dreamin/shiroa/pull/177
- Set up package tests in https://github.com/Myriad-Dreamin/shiroa/pull/121
- Removed cross-link protocol usages in https://github.com/Myriad-Dreamin/shiroa/pull/130 and https://github.com/Myriad-Dreamin/shiroa/pull/159

### CLI

- (Test) Added and testing a minimal project to be used in `shiroa init` section in https://github.com/Myriad-Dreamin/shiroa/pull/214
- (Change) Deprecating `-w` flag in https://github.com/Myriad-Dreamin/shiroa/pull/215
- Hot reloading necessary documents and themes in https://github.com/Myriad-Dreamin/shiroa/pull/158
- Pretty printing compilation in https://github.com/Myriad-Dreamin/shiroa/pull/124
- Ignoring the infamous foreign object warning in https://github.com/Myriad-Dreamin/shiroa/pull/125
- Fully parallelized compilation in https://github.com/Myriad-Dreamin/shiroa/pull/126
- Watching compilation when serving in https://github.com/Myriad-Dreamin/shiroa/pull/127
- Updated title formatting by @Enter-tainer in https://github.com/Myriad-Dreamin/shiroa/pull/152
- Using TextExport that respects style tag in https://github.com/Myriad-Dreamin/shiroa/pull/154
- Initializing page.typ with correct book.typ path by @Sinofine in https://github.com/Myriad-Dreamin/shiroa/pull/174
- Not longer generating description from CLI in https://github.com/Myriad-Dreamin/shiroa/pull/210

### Misc

- Ensured wasm module matches frontend build by @kxxt in https://github.com/Myriad-Dreamin/shiroa/pull/101
- Updated README.md and installation.typ by @HoiGe in https://github.com/Myriad-Dreamin/shiroa/pull/119
- Update doucmentation about support section and todo list in https://github.com/Myriad-Dreamin/shiroa/pull/137
- Refactored rendering logic by @QuadnucYard in https://github.com/Myriad-Dreamin/shiroa/pull/155 and https://github.com/Myriad-Dreamin/shiroa/pull/165
- Publishing typst packages by workflow in https://github.com/Myriad-Dreamin/shiroa/pull/200, https://github.com/Myriad-Dreamin/shiroa/pull/201, and https://github.com/Myriad-Dreamin/shiroa/pull/202
- (Docs) Fixed incorrect argument name in `shiroa help build` by @xiyihan0 in https://github.com/Myriad-Dreamin/shiroa/pull/220
- (Docs) Fixed book.toml => book.typ typo by @kaliiiiiiiiii in https://github.com/Myriad-Dreamin/shiroa/pull/204
- (Docs) Updated book metadata descriptions and enhance theme docs in https://github.com/Myriad-Dreamin/shiroa/pull/218
- (Docs) Updated introduction, FAQ, get started, and installation guides in https://github.com/Myriad-Dreamin/shiroa/pull/213
- (Docs) Turned en-dash into two hypen (long flag) in https://github.com/Myriad-Dreamin/shiroa/pull/208

**Full Changelog**: https://github.com/Myriad-Dreamin/shiroa/compare/v0.3.0...v0.3.1

## v0.3.0 - [2025-03-05]

- Bumped `@preview/shiroa` package to 0.2.0 in https://github.com/Myriad-Dreamin/shiroa/pull/103
- Bumped typst to v0.13.0 in https://github.com/Myriad-Dreamin/shiroa/pull/97 and https://github.com/Myriad-Dreamin/shiroa/pull/103

### CLI Tool

- Added cross-page search component in https://github.com/Myriad-Dreamin/shiroa/pull/99
- Rendering pages into static html using HTML export with `--mode=static-html` in https://github.com/Myriad-Dreamin/shiroa/pull/103 and https://github.com/Myriad-Dreamin/shiroa/pull/105

**Full Changelog**: https://github.com/Myriad-Dreamin/shiroa/compare/v0.2.0...v0.3.0
