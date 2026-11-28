//! The pixelarticons glyphs the launcher draws.
//!
//! Only the icons that are actually used are compiled in: each one is a
//! `include_bytes!` of a single vendored SVG, so the binary carries the
//! documents it paints and no icon font, asset tree or runtime lookup. Source,
//! pinned commit, license and the (empty) list of changes are in
//! `assets/icons/NOTICE.md`.
//!
//! The art is 24×24 and is always drawn at [`layout::ICON`], which is the same
//! 24 px: one art pixel is one screen pixel, so resvg never resamples a stroke
//! and the pixel edges stay square.

use gpui_kit::{px, rgb, svg, IntoElement, Styled};

use crate::design_tokens::layout;

/// One glyph from the vendored set.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Icon {
    /// A page: home, account, download, dependencies, music, mods, settings.
    Home,
    User,
    Download,
    Package,
    Music,
    Blocks,
    Settings,
    /// The music transport.
    Play,
    Pause,
    Volume,
    /// The settings sections: appearance (accent swatches), memory, the
    /// download proxy, and the disk sweep.
    Swatch,
    Memory,
    Trash,
}

impl Icon {
    /// Every icon, for the test that guards the grid they are drawn on.
    #[cfg(test)]
    const ALL: [Icon; 13] = [
        Icon::Home,
        Icon::User,
        Icon::Download,
        Icon::Package,
        Icon::Music,
        Icon::Blocks,
        Icon::Settings,
        Icon::Play,
        Icon::Pause,
        Icon::Volume,
        Icon::Swatch,
        Icon::Memory,
        Icon::Trash,
    ];

    /// The SVG document, byte for byte as upstream ships it.
    ///
    /// Listing them by hand is the point: an icon that is not named here is not
    /// in the executable.
    const fn data(self) -> &'static [u8] {
        match self {
            Icon::Home => include_bytes!("../../../assets/icons/home.svg"),
            Icon::User => include_bytes!("../../../assets/icons/user.svg"),
            Icon::Download => include_bytes!("../../../assets/icons/download.svg"),
            Icon::Package => include_bytes!("../../../assets/icons/package.svg"),
            Icon::Music => include_bytes!("../../../assets/icons/music.svg"),
            Icon::Blocks => include_bytes!("../../../assets/icons/blocks.svg"),
            Icon::Settings => include_bytes!("../../../assets/icons/settings-2.svg"),
            Icon::Play => include_bytes!("../../../assets/icons/play.svg"),
            Icon::Pause => include_bytes!("../../../assets/icons/pause.svg"),
            Icon::Volume => include_bytes!("../../../assets/icons/volume-2.svg"),
            Icon::Swatch => include_bytes!("../../../assets/icons/colors-swatch.svg"),
            Icon::Memory => include_bytes!("../../../assets/icons/memory-stick.svg"),
            Icon::Trash => include_bytes!("../../../assets/icons/trash.svg"),
        }
    }
}

/// One icon, tinted `color`.
///
/// gpui rasterizes the SVG into an alpha mask and paints it with the color it
/// is given, so the same file serves every state: the caller decides whether
/// that is a muted gray or the accent.
pub(crate) fn icon(kind: Icon, color: u32) -> impl IntoElement {
    svg()
        .data(Icon::data(kind))
        .w(px(layout::ICON))
        .h(px(layout::ICON))
        .flex_shrink_0()
        .text_color(rgb(color))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every vendored file is drawn on a 24×24 canvas, which is what makes
    /// rendering it at `layout::ICON` a 1:1 copy instead of a resample.
    ///
    /// The `viewBox` is the geometry: upstream minifies a few files (one of them
    /// ships without `width`/`height`), and usvg sizes a document from its
    /// viewBox, so those attributes are checked only when present.
    #[test]
    fn every_icon_is_drawn_on_the_grid_it_is_rendered_at() {
        for kind in Icon::ALL {
            let document = std::str::from_utf8(kind.data()).expect("an SVG is UTF-8");
            for attribute in [
                r#"viewBox="0 0 24 24""#,
                // `currentColor` is what lets one file serve every tint.
                r#"fill="currentColor""#,
            ] {
                assert!(document.contains(attribute), "{kind:?} lacks {attribute}");
            }
            for attribute in [r#"width="24""#, r#"height="24""#] {
                assert!(
                    document.contains(attribute) || !document.contains("width="),
                    "{kind:?} declares a size that is not 24x24"
                );
            }
        }
        assert_eq!(layout::ICON, 24.0);
    }
}
