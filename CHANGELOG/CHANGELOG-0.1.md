# v0.1.3

## Changelog since v0.1.3

**Full Changelog**: https://github.com/Myriad-Dreamin/typst-book/compare/v0.1.2...v0.1.3

Most efforts are internal improvements and there is no external changes since v0.1.2.

- Implemented new text selection, which already works great on simple pages.
- Improved performance on large pages. Note: you may still get bad performance if you set `#set page(height: auto)`, which will get improved in the future.

# v0.1.2

## Changelog since v0.1.2

**Full Changelog**: https://github.com/Myriad-Dreamin/typst-book/compare/v0.1.1...v0.1.2

## Feature

- feat: automatically assign section number in https://github.com/Myriad-Dreamin/typst-book/pull/37
- dev: enable ligature feature in https://github.com/Myriad-Dreamin/typst-book/pull/38
- scripting: cross link support in https://github.com/Myriad-typstbook/pull/41
- scripting: support semantic link jump in https://github.mMyriad-Dreamin/typst-book/pull/42

## Enhancement

- theme: override target="\_blank" behavior in https://github.com/Myriad-Dreamin/typst-book/pull/27 and https://github.com/Myriad-Dreamin/typst-book/pull/28
- scripting: improve plain text conversion in https://github.com/Myriad-Dreamin/typst-book/pull/39
  - This is used by conversion of typst title contents
- scripting: don't justify code block in https://github.com/Myriad-Dreamin/typst-book/pull/40
  - You can update your template like it.
- build: upgrade typst.ts to 0.4.1 in https://github.com/Myriad-Dreamin/typst-book/pull/36
  - It brings text selection enhancement

# v0.1.1

## Changelog since v0.1.1

**Full Changelog**: https://github.com/Myriad-Dreamin/typst-book/compare/v0.1.0...v0.1.1

## Enhancement

- cli: correctly evict compilation cache in https://github.com/Myriad-Dreamin/typst-book/commit/149446ab63dd9ea628b1d30bc5eed7cac1582b62
  - this reduces memory usage slightly.

## Feature

- theme: sidebar improvement in https://github.com/Myriad-Dreamin/typst-book/commit/dfff00639142d881cd11a8ae2da379aa08505b0b and https://github.com/Myriad-Dreamin/typst-book/commit/313c11d37df679670426e85c05431b687aa71056

- theme: scrollbar improvement in https://github.com/Myriad-Dreamin/typst-book/commit/e274777809a6fc469f4b84509cf5522d94bc9daf

- theme: support dark themes and more by @seven-mile in https://github.com/Myriad-Dreamin/typst-book/pull/23

- typesetting: add repository-edit template in https://github.com/Myriad-Dreamin/typst-book/commit/9f1260c0706954faeb7bb90388143cdbf11185ab

- cli: add version command in https://github.com/Myriad-Dreamin/typst-book/pull/25

# v0.1.0

Initial release
