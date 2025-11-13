//! Render a QR code as qpui elements
//!
//! A simple Component to render a QR code as qpui elements with gpui-native APIs for
//! customization and flexibility.
//!
//! To render a QR code just pass the size and a vector of booleans representing the data, where
//! true means this block needs to be rendered as a dark square. The size indicates how many columns
//! to sort the qrcode into, thus the data should usually have the a length of size squared.
//!
//! Examples:
//! ```
//! use gpui_qrcode::QrCode;
//! use gpui::prelude::*;
//!
//! let qr_code = QrCode::new(10, vec![true, false, true, false]);
//! ```
//!
//! You can also customize the appearance of the QR code by applying styles:
//! ```
//! use gpui_qrcode::QrCode;
//! use gpui::prelude::*;
//!
//! let qr_code = QrCode::new(10, vec![true, false, true, false])
//!     .bg(gpui::blue())
//!     .p_2();
//! ```
//!
//! To customize the appearance of the QR code dots use the `refine_dot_style` method:
//! ```
//! use gpui_qrcode::QrCode;
//! use gpui::{prelude::*, StyleRefinement};
//!
//! let qr_code = QrCode::new(10, vec![true, false, true, false])
//!     .bg(gpui::blue())
//!     .p_2()
//!     .refine_dot_style(&StyleRefinement {
//!     background: Some(gpui::red().into()),
//!     ..Default::default()
//! });
//! ```
//!

use gpui::{
    AbsoluteLength, DefiniteLength, IntoElement, Length, ParentElement, Refineable, RenderOnce,
    SizeRefinement, StyleRefinement, Styled, div,
};

#[cfg(feature = "qrcode-support")]
mod qrcode_lib_support {
    /// Support for rendering QR codes directly from `qrcode::QrCode`.

    #[derive(Debug, thiserror::Error)]
    pub enum QrCodeRenderError {
        #[error("Exceeded width of u16::MAX")]
        ExceededWidth,
    }

    impl TryFrom<qrcode::QrCode> for super::QrCode {
        type Error = QrCodeRenderError;

        fn try_from(data: qrcode::QrCode) -> Result<Self, Self::Error> {
            let width =
                u16::try_from(data.width()).map_err(|_| QrCodeRenderError::ExceededWidth)?;
            Ok(Self::new(
                width,
                data.into_colors()
                    .into_iter()
                    .map(|color| matches!(color, qrcode::Color::Dark))
                    .collect(),
            ))
        }
    }
}

/// A simple Component to render a QR code as qpui elements
///
/// This is a `Styled` component, so you can use the regular styling methods of
/// gpui like `.bg(gpui::blue())` to set the background or `.p_2()` to add padding.
///
/// To change the color and behavior of the dots apply the styles you want via the
/// `refine_dot_style` method.
#[derive(Clone, Debug, PartialEq, IntoElement)]
pub struct QrCode {
    cols: u16,
    data: Vec<bool>,
    style: StyleRefinement,
    dot_style: StyleRefinement,
}

impl QrCode {
    /// Create a new QR code component with of `cols` dots per row.
    ///
    /// For each entry in `data`, a dot will be rendered if the entry is `true`, thus
    /// the data is expected to be of length `cols * cols` (but that isn't enforced).
    pub fn new(cols: u16, data: Vec<bool>) -> Self {
        Self {
            cols,
            data,

            style: StyleRefinement {
                background: Some(gpui::white().into()),
                ..Default::default()
            },
            dot_style: StyleRefinement {
                background: Some(gpui::black().into()),
                min_size: SizeRefinement {
                    width: Some(Length::Definite(DefiniteLength::Absolute(
                        AbsoluteLength::Rems(gpui::Rems(0.25)),
                    ))),
                    height: Some(Length::Definite(DefiniteLength::Absolute(
                        AbsoluteLength::Rems(gpui::Rems(0.25)),
                    ))),
                },
                ..Default::default()
            },
        }
    }

    /// Refine the style of the dots in the QR code.
    ///
    /// For example, you can set the background color to red like this:
    ///
    /// ```rust
    /// # use gpui_qrcode::QrCode;
    /// # use gpui::StyleRefinement;
    /// let qr_code = QrCode::new(10, vec![true; 100]);
    /// let qr_code = qr_code.refine_dot_style(&StyleRefinement {
    ///     background: Some(gpui::red().into()),
    ///     ..Default::default()
    /// });
    /// ```
    pub fn refine_dot_style(mut self, new_styles: &StyleRefinement) -> Self {
        self.dot_style.refine(new_styles);
        self
    }
}

impl Styled for QrCode {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for QrCode {
    fn render(self, _window: &mut gpui::Window, _cx: &mut gpui::App) -> impl gpui::IntoElement {
        let mut d = div();
        d.style().refine(&self.style);

        d.grid()
            .grid_cols(self.cols)
            .children(self.data.into_iter().map(|is_dark| {
                let mut d = div();
                if is_dark {
                    d.style().refine(&self.dot_style)
                }
                d
            }))
    }
}
