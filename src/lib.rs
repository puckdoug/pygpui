use gpui::*;
use pyo3::prelude::*;

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
#[pyo3(signature = (a = 10, b = 10))]
fn hello(a: usize, b: usize) -> PyResult<String> {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| HelloWorld {
                text: "World".into(),
            })
        })
        .unwrap();
    });

    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule(name = "pygpui")]
fn pygpui(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    Ok(())
}
