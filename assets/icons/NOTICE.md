# Vendored icons

The sidebar draws seven pixelarticons glyphs. Only those seven files are
compiled in (`include_bytes!` in `crates/tessera-gui/src/icons.rs`), so the
binary carries exactly the artwork it shows — no icon font, no runtime asset
lookup, no `assets/` tree at startup.

- Upstream: <https://github.com/halfmage/pixelarticons> — site
  <https://pixelarticons.com>
- Pinned commit: `c9ecc2da61afde85dbe539025e3d931c6152ad75` (tag `2.4.2`)
- Designer: Gerrit Halfmann (halfmage)
- License: MIT — the full text is in `LICENSE`.
- Taken, unchanged: `svg/home.svg`, `svg/user.svg`, `svg/download.svg`,
  `svg/package.svg`, `svg/music.svg`, `svg/blocks.svg`, `svg/settings-2.svg`.
- Changes: none. The artwork, the file names and the `<path>` data are
  upstream's, byte for byte.
- Only the **free** set was taken. pixelarticons also sells a Pro set; nothing
  from it is in this repository.

## Why the size is fixed

The art is drawn on a 24×24 grid of axis-aligned 1 px rectangles and every file
declares `width="24" height="24" viewBox="0 0 24 24"`. `layout::ICON` is 24 px,
which is also the line box of 16 px body text at the 1.5 line height, so an
icon and the label beside it are the same height by construction.

At that size one art pixel is one screen pixel: resvg has nothing to resample,
so the edges stay square. Rendered at any other size the strokes land on
fractions of a pixel and go soft — a different size wants a different set, not
a scaled copy of this one.
