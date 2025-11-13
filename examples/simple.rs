use gpui::{
    AbsoluteLength, App, Application, Context, CornersRefinement, DefiniteLength, Div, Fill,
    Length, SharedString, SizeRefinement, StyleRefinement, WeakEntity, Window, WindowOptions, div,
    prelude::*, px, rgb,
};
use gpui_qrcode::QrCode;

struct SimpleExample {
    inner: QrCode,
    dot_style: StyleRefinement,
}

impl SimpleExample {
    fn new(inner: QrCode) -> Self {
        Self {
            inner,
            dot_style: StyleRefinement {
                background: Some(Fill::from(gpui::black())),
                ..Default::default()
            },
        }
    }

    fn render_dot_colors(me: WeakEntity<Self>) -> Div {
        div()
            .flex()
            .gap_2()
            .child(SharedString::new("Color: "))
            .children(
                [
                    gpui::red(),
                    gpui::green(),
                    gpui::blue(),
                    gpui::yellow(),
                    gpui::black(),
                ]
                .into_iter()
                .enumerate()
                .map(|(index, color)| {
                    let me = me.clone();
                    div()
                        .id(SharedString::new(format!("color-{index}")))
                        .size_8()
                        .bg(color.clone())
                        .on_click(move |_ev, _win, cx| {
                            let _ = me.update(cx, |this, cx| {
                                this.dot_style.background = Some(Fill::from(color));
                                cx.notify()
                            });
                        })
                }),
            )
    }

    fn render_dot_sizes(me: WeakEntity<Self>) -> Div {
        div()
            .flex()
            .gap_2()
            .child(SharedString::new("Dot Size: "))
            .children([0.25f32, 0.5f32, 1f32, 2f32].into_iter().map(|size| {
                let me = me.clone();
                div()
                    .id(SharedString::new(format!("size-{size}")))
                    .child(SharedString::new(format!("{size}rem")))
                    .on_click(move |_ev, _win, cx| {
                        let _ = me.update(cx, |this, cx| {
                            this.dot_style.size = SizeRefinement {
                                width: Some(Length::Definite(DefiniteLength::Absolute(
                                    AbsoluteLength::Rems(gpui::Rems(size)),
                                ))),
                                height: Some(Length::Definite(DefiniteLength::Absolute(
                                    AbsoluteLength::Rems(gpui::Rems(size)),
                                ))),
                            };
                            cx.notify();
                        });
                    })
            }))
    }

    fn render_pad_sizes(me: WeakEntity<Self>) -> Div {
        div()
            .flex()
            .gap_2()
            .child(SharedString::new("Padding: "))
            .children([0.25f32, 0.5f32, 1f32, 2f32].into_iter().map(|size| {
                let me = me.clone();
                div()
                    .id(SharedString::new(format!("padding-size-{size}")))
                    .child(SharedString::new(format!("{size}rem")))
                    .on_click(move |_ev, _win, cx| {
                        let _ = me.update(cx, |this, cx| {
                            this.inner = this.inner.clone().p(DefiniteLength::Absolute(
                                AbsoluteLength::Rems(gpui::Rems(size)),
                            ));
                            cx.notify();
                        });
                    })
            }))
    }

    fn render_dot_style(me: WeakEntity<Self>) -> Div {
        div()
            .flex()
            .gap_2()
            .child(SharedString::new("Dot Style: "))
            .children(
                [
                    StyleRefinement {
                        corner_radii: CornersRefinement {
                            top_left: Some(AbsoluteLength::Pixels(px(10.0))),
                            top_right: Some(AbsoluteLength::Pixels(px(10.0))),
                            bottom_left: Some(AbsoluteLength::Pixels(px(10.0))),
                            bottom_right: Some(AbsoluteLength::Pixels(px(10.0))),
                        },
                        ..Default::default()
                    },
                    StyleRefinement {
                        corner_radii: CornersRefinement {
                            top_left: Some(AbsoluteLength::Pixels(px(10.0))),
                            top_right: Some(AbsoluteLength::Pixels(px(10.0))),
                            bottom_left: Some(AbsoluteLength::Pixels(px(10.0))),
                            bottom_right: Some(AbsoluteLength::Pixels(px(0.0))),
                        },
                        ..Default::default()
                    },
                    StyleRefinement {
                        corner_radii: CornersRefinement {
                            top_left: Some(AbsoluteLength::Pixels(px(0.0))),
                            top_right: Some(AbsoluteLength::Pixels(px(10.0))),
                            bottom_left: Some(AbsoluteLength::Pixels(px(10.0))),
                            bottom_right: Some(AbsoluteLength::Pixels(px(10.0))),
                        },
                        ..Default::default()
                    },
                ]
                .into_iter()
                .enumerate()
                .map(|(index, style)| {
                    let me = me.clone();
                    let mut d = div()
                        .id(SharedString::new(format!("style-{index}")))
                        .bg(gpui::black())
                        .size_5();

                    d.style().refine(&style);
                    d.on_click(move |_ev, _win, cx| {
                        let _ = me.update(cx, |this, cx| {
                            this.dot_style.refine(&style);
                            cx.notify();
                        });
                    })
                }),
            )
    }

    fn render_actions(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let me = cx.entity().downgrade();
        div()
            .flex()
            .flex_col()
            .items_center()
            .child(SharedString::new("Configuration"))
            .child(Self::render_dot_colors(me.clone()))
            .child(Self::render_dot_sizes(me.clone()))
            .child(Self::render_dot_style(me.clone()))
            .child(Self::render_pad_sizes(me.clone()))
        // .child()
    }
}

impl Render for SimpleExample {
    fn render(&mut self, win: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .size_full()
            .content_stretch()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(self.inner.clone().refine_dot_style(&self.dot_style))
            .child(self.render_actions(win, cx))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(
            WindowOptions {
                ..Default::default()
            },
            |_, cx| {
                let q =
                    QrCode::try_from(qrcode::QrCode::new("https://hellozoe.app").unwrap()).unwrap();
                cx.new(|_| SimpleExample::new(q))
            },
        )
        .unwrap();
    });
}
