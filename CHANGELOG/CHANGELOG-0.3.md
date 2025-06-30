# Change Log

All notable changes to the "shiroa" will be documented in this file.

Check [Keep a Changelog](http://keepachangelog.com/) for recommendations on how to structure this file.

## v0.3.1 - [2025-06-30]

The diff between v0.3.1-rc2 and v0.3.1-rc3 is rendered at the [section](#diff-v031-rc2v031-rc3).

- Bumped shiroa package to v0.2.3 in https://github.com/Myriad-Dreamin/shiroa/pull/131 and https://github.com/Myriad-Dreamin/shiroa/pull/141
- Bumped zebraw to v0.5.2 in https://github.com/Myriad-Dreamin/shiroa/pull/139

### Theme

- feat(theme): introduce startlight in https://github.com/Myriad-Dreamin/shiroa/pull/157, https://github.com/Myriad-Dreamin/shiroa/pull/160, https://github.com/Myriad-Dreamin/shiroa/pull/161, https://github.com/Myriad-Dreamin/shiroa/pull/162, https://github.com/Myriad-Dreamin/shiroa/pull/163, https://github.com/Myriad-Dreamin/shiroa/pull/164, and https://github.com/Myriad-Dreamin/shiroa/pull/166

### Package

- (Fix) Corrected show rule for equations in https://github.com/Myriad-Dreamin/shiroa/pull/120
- (Fix) Strengthened `plain-text` implementation in https://github.com/Myriad-Dreamin/shiroa/pull/122 and https://github.com/Myriad-Dreamin/shiroa/pull/138
- Set up package tests in https://github.com/Myriad-Dreamin/shiroa/pull/121
- Removed cross-link protocol usages in https://github.com/Myriad-Dreamin/shiroa/pull/130 and https://github.com/Myriad-Dreamin/shiroa/pull/159

### CLI

- Hot reloading necessary documents and themes in https://github.com/Myriad-Dreamin/shiroa/pull/158
- Pretty printing compilation in https://github.com/Myriad-Dreamin/shiroa/pull/124
- Ignoring the infamous foreign object warning in https://github.com/Myriad-Dreamin/shiroa/pull/125
- Fully parallelized compilation in https://github.com/Myriad-Dreamin/shiroa/pull/126
- Watching compilation when serving in https://github.com/Myriad-Dreamin/shiroa/pull/127
- Updated title formatting by @Enter-tainer in https://github.com/Myriad-Dreamin/shiroa/pull/152
- Using TextExport that respects style tag in https://github.com/Myriad-Dreamin/shiroa/pull/154

### Misc

- Updated README.md and installation.typ by @HoiGe in https://github.com/Myriad-Dreamin/shiroa/pull/119
- Ensured wasm module matches frontend build by @kxxt in https://github.com/Myriad-Dreamin/shiroa/pull/101
- Update doucmentation about support section and todo list in https://github.com/Myriad-Dreamin/shiroa/pull/137
- Refactored rendering logic by @QuadnucYard in https://github.com/Myriad-Dreamin/shiroa/pull/155 and https://github.com/Myriad-Dreamin/shiroa/pull/165

### Diff v0.3.1-rc2...v0.3.1-rc3

```diff
-## v0.3.1 - [2025-04-25]
+## v0.3.1 - [2025-06-30]
 
+### Theme
+
+- feat(theme): introduce startlight in https://github.com/Myriad-Dreamin/shiroa/pull/157, https://github.com/Myriad-Dreamin/shiroa/pull/160, https://github.com/Myriad-Dreamin/shiroa/pull/161, https://github.com/Myriad-Dreamin/shiroa/pull/162, https://github.com/Myriad-Dreamin/shiroa/pull/163, https://github.com/Myriad-Dreamin/shiroa/pull/164, and https://github.com/Myriad-Dreamin/shiroa/pull/166
+
 ### Package
 
 - (Fix) Corrected show rule for equations in https://github.com/Myriad-Dreamin/shiroa/pull/120
 - (Fix) Strengthened `plain-text` implementation in https://github.com/Myriad-Dreamin/shiroa/pull/122 and https://github.com/Myriad-Dreamin/shiroa/pull/138
 - Set up package tests in https://github.com/Myriad-Dreamin/shiroa/pull/121
-- Supported `cross-link` in HTML export in https://github.com/Myriad-Dreamin/shiroa/pull/130
+- Removed cross-link protocol usages in https://github.com/Myriad-Dreamin/shiroa/pull/130 and https://github.com/Myriad-Dreamin/shiroa/pull/159
 
 ### CLI
 
+- Hot reloading necessary documents and themes in https://github.com/Myriad-Dreamin/shiroa/pull/158
 - Pretty printing compilation in https://github.com/Myriad-Dreamin/shiroa/pull/124
 - Ignoring the infamous foreign object warning in https://github.com/Myriad-Dreamin/shiroa/pull/125
 - Fully parallelized compilation in https://github.com/Myriad-Dreamin/shiroa/pull/126
 - Watching compilation when serving in https://github.com/Myriad-Dreamin/shiroa/pull/127
+- Updated title formatting by @Enter-tainer in https://github.com/Myriad-Dreamin/shiroa/pull/152
+- Using TextExport that respects style tag in https://github.com/Myriad-Dreamin/shiroa/pull/154
 
 ### Misc
 
 - Updated README.md and installation.typ by @HoiGe in https://github.com/Myriad-Dreamin/shiroa/pull/119
 - Ensured wasm module matches frontend build by @kxxt in https://github.com/Myriad-Dreamin/shiroa/pull/101
 - Update doucmentation about support section and todo list in https://github.com/Myriad-Dreamin/shiroa/pull/137
+- Refactored rendering logic by @QuadnucYard in https://github.com/Myriad-Dreamin/shiroa/pull/155 and https://github.com/Myriad-Dreamin/shiroa/pull/165
```

### Diff v0.3.1-rc1...v0.3.1-rc2

```diff
-## v0.3.1 - [2025-03-30]
+## v0.3.1 - [2025-04-25]

-- Bumped shiroa package to v0.2.2 in https://github.com/Myriad-Dreamin/shiroa/pull/131
+- Bumped shiroa package to v0.2.3 in https://github.com/Myriad-Dreamin/shiroa/pull/131 and https://github.com/Myriad-Dreamin/shiroa/pull/141
+- Bumped zebraw to v0.5.2 in https://github.com/Myriad-Dreamin/shiroa/pull/139

 ### Package

 - (Fix) Corrected show rule for equations in https://github.com/Myriad-Dreamin/shiroa/pull/120
-- (Fix) Strengthened `plain-text` implementation in https://github.com/Myriad-Dreamin/shiroa/pull/122
+- (Fix) Strengthened `plain-text` implementation in https://github.com/Myriad-Dreamin/shiroa/pull/122 and https://github.com/Myriad-Dreamin/shiroa/pull/138
 - Set up package tests in https://github.com/Myriad-Dreamin/shiroa/pull/121
 - Supported `cross-link` in HTML export in https://github.com/Myriad-Dreamin/shiroa/pull/130

@@ -26,6 +27,7 @@ Check [Keep a Changelog](http://keepachangelog.com/) for recommendations on how

 - Updated README.md and installation.typ by @HoiGe in https://github.com/Myriad-Dreamin/shiroa/pull/119
 - Ensured wasm module matches frontend build by @kxxt in https://github.com/Myriad-Dreamin/shiroa/pull/101
+- Update doucmentation about support section and todo list in https://github.com/Myriad-Dreamin/shiroa/pull/137
```

**Full Changelog**: https://github.com/Myriad-Dreamin/shiroa/compare/v0.3.0...v0.3.1

## v0.3.0 - [2025-03-05]

- Bumped `@preview/shiroa` package to 0.2.0 in https://github.com/Myriad-Dreamin/shiroa/pull/103
- Bumped typst to v0.13.0 in https://github.com/Myriad-Dreamin/shiroa/pull/97 and https://github.com/Myriad-Dreamin/shiroa/pull/103

### CLI Tool

- Added cross-page search component in https://github.com/Myriad-Dreamin/shiroa/pull/99
- Rendering pages into static html using HTML export with `--mode=static-html` in https://github.com/Myriad-Dreamin/shiroa/pull/103 and https://github.com/Myriad-Dreamin/shiroa/pull/105

**Full Changelog**: https://github.com/Myriad-Dreamin/shiroa/compare/v0.2.0...v0.3.0
