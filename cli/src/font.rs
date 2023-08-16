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
];

#[cfg(not(feature = "embedded-fonts"))]
pub static EMBEDDED_FONT: &[Cow<'_, [u8]>] = &[];
