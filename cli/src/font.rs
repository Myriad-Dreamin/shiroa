use std::borrow::Cow;

macro_rules! font {
    ($filename:literal) => {
        include_bytes!(concat!("../../assets/fonts/", $filename)).as_slice()
    };
}

#[cfg(feature = "embedded-fonts")]
pub static EMBEDDED_FONT: &[Cow<'_, [u8]>] = &[
    // Embed default fonts.
    Cow::Borrowed(font!("LinLibertine_R.ttf")),
    Cow::Borrowed(font!("LinLibertine_RB.ttf")),
    Cow::Borrowed(font!("LinLibertine_RBI.ttf")),
    Cow::Borrowed(font!("LinLibertine_RI.ttf")),
    Cow::Borrowed(font!("NewCMMath-Book.otf")),
    Cow::Borrowed(font!("NewCMMath-Regular.otf")),
    Cow::Borrowed(font!("NewCM10-Regular.otf")),
    Cow::Borrowed(font!("NewCM10-Bold.otf")),
    Cow::Borrowed(font!("NewCM10-Italic.otf")),
    Cow::Borrowed(font!("NewCM10-BoldItalic.otf")),
    Cow::Borrowed(font!("DejaVuSansMono.ttf")),
    Cow::Borrowed(font!("DejaVuSansMono-Bold.ttf")),
    Cow::Borrowed(font!("DejaVuSansMono-Oblique.ttf")),
    Cow::Borrowed(font!("DejaVuSansMono-BoldOblique.ttf")),
    // Embed CJK fonts.
    #[cfg(feature = "embedded-cjk-fonts")]
    Cow::Borrowed(font!("InriaSerif-Bold.ttf")),
    #[cfg(feature = "embedded-cjk-fonts")]
    Cow::Borrowed(font!("InriaSerif-BoldItalic.ttf")),
    #[cfg(feature = "embedded-cjk-fonts")]
    Cow::Borrowed(font!("InriaSerif-Italic.ttf")),
    #[cfg(feature = "embedded-cjk-fonts")]
    Cow::Borrowed(font!("InriaSerif-Regular.ttf")),
    #[cfg(feature = "embedded-cjk-fonts")]
    Cow::Borrowed(font!("Roboto-Regular.ttf")),
    #[cfg(feature = "embedded-cjk-fonts")]
    Cow::Borrowed(font!("NotoSerifCJKsc-Regular.otf")),
    // Embed emoji fonts.
    #[cfg(feature = "embedded-emoji-fonts")]
    Cow::Borrowed(font!("TwitterColorEmoji.ttf")),
    #[cfg(feature = "embedded-emoji-fonts")]
    Cow::Borrowed(font!("NotoColorEmoji.ttf")),
];

#[cfg(not(feature = "embedded-fonts"))]
pub static EMBEDDED_FONT: &[Cow<'_, [u8]>] = &[];
