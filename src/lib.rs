use gpui::{
    AbsoluteLength, DefiniteLength, IntoElement, Length, ParentElement, Refineable, RenderOnce,
    SizeRefinement, StyleRefinement, Styled, div,
};

#[cfg(feature = "qrcode-support")]
mod qrcode_lib_support {

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

// A simple Component to render a QR code as qpui elements
#[derive(Clone, Debug, PartialEq, IntoElement)]
pub struct QrCode {
    width: u16,
    data: Vec<bool>,
    style: StyleRefinement,
    dot_style: StyleRefinement,
}

impl QrCode {
    pub fn new(width: u16, data: Vec<bool>) -> Self {
        Self {
            width,
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
            .grid_cols(self.width)
            .children(self.data.into_iter().map(|is_dark| {
                let mut d = div();
                if is_dark {
                    d.style().refine(&self.dot_style)
                }
                d
            }))
    }
}
