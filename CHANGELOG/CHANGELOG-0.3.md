# Change Log

All notable changes to the "shiroa" will be documented in this file.

Check [Keep a Changelog](http://keepachangelog.com/) for recommendations on how to structure this file.

## v0.3.1 - [2025-04-25]

The diff between v0.3.1-rc1 and v0.3.1-rc2 is rendered at the [section](#diff-v031-rc1v031-rc2).

- Bumped shiroa package to v0.2.3 in https://github.com/Myriad-Dreamin/shiroa/pull/131 and https://github.com/Myriad-Dreamin/shiroa/pull/141
- Bumped zebraw to v0.5.2 in https://github.com/Myriad-Dreamin/shiroa/pull/139

### Package

- (Fix) Corrected show rule for equations in https://github.com/Myriad-Dreamin/shiroa/pull/120
- (Fix) Strengthened `plain-text` implementation in https://github.com/Myriad-Dreamin/shiroa/pull/122 and https://github.com/Myriad-Dreamin/shiroa/pull/138
- Set up package tests in https://github.com/Myriad-Dreamin/shiroa/pull/121
- Supported `cross-link` in HTML export in https://github.com/Myriad-Dreamin/shiroa/pull/130

### CLI

- Pretty printing compilation in https://github.com/Myriad-Dreamin/shiroa/pull/124
- Ignoring the infamous foreign object warning in https://github.com/Myriad-Dreamin/shiroa/pull/125
- Fully parallelized compilation in https://github.com/Myriad-Dreamin/shiroa/pull/126
- Watching compilation when serving in https://github.com/Myriad-Dreamin/shiroa/pull/127

### Misc

- Updated README.md and installation.typ by @HoiGe in https://github.com/Myriad-Dreamin/shiroa/pull/119
- Ensured wasm module matches frontend build by @kxxt in https://github.com/Myriad-Dreamin/shiroa/pull/101
- Update doucmentation about support section and todo list in https://github.com/Myriad-Dreamin/shiroa/pull/137

**Full Changelog**: https://github.com/Myriad-Dreamin/shiroa/compare/v0.3.0...v0.3.1

## v0.3.0 - [2025-03-05]

- Bumped `@preview/shiroa` package to 0.2.0 in https://github.com/Myriad-Dreamin/shiroa/pull/103
- Bumped typst to v0.13.0 in https://github.com/Myriad-Dreamin/shiroa/pull/97 and https://github.com/Myriad-Dreamin/shiroa/pull/103

### CLI Tool

- Added cross-page search component in https://github.com/Myriad-Dreamin/shiroa/pull/99
- Rendering pages into static html using HTML export with `--mode=static-html` in https://github.com/Myriad-Dreamin/shiroa/pull/103 and https://github.com/Myriad-Dreamin/shiroa/pull/105

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

 **Full Changelog**: https://github.com/Myriad-Dreamin/shiroa/compare/v0.3.0...v0.3.1
```

**Full Changelog**: https://github.com/Myriad-Dreamin/shiroa/compare/v0.2.0...v0.3.0
