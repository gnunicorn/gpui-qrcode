use gpui::{IntoElement, ParentElement, RenderOnce, Styled, div, prelude::FluentBuilder};
use qrcode::QrCode;

#[derive(Debug, thiserror::Error)]
pub enum QrCodeRenderError {
    #[error("Exceeded width of u16::MAX")]
    ExceededWidth,
}

#[derive(Clone, Debug, PartialEq, Eq, IntoElement)]
pub struct QrCodeComponent {
    width: u16,
    data: Vec<qrcode::Color>,
}

impl QrCodeComponent {
    pub fn prepare(data: QrCode) -> Result<Self, QrCodeRenderError> {
        let width = u16::try_from(data.width()).map_err(|_| QrCodeRenderError::ExceededWidth)?;
        Ok(Self {
            width,
            data: data.into_colors(),
        })
    }
}

impl RenderOnce for QrCodeComponent {
    fn render(self, _window: &mut gpui::Window, _cx: &mut gpui::App) -> impl gpui::IntoElement {
        div()
            .bg(gpui::white())
            .grid()
            .grid_cols(self.width)
            .children(self.data.into_iter().map(|color| {
                div()
                    .min_h_1()
                    .min_w_1()
                    .when(color == qrcode::Color::Dark, |div| div.bg(gpui::black()))
            }))
    }
}
