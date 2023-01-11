use std::fmt::Write;

pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{}", &buffer);
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

pub struct Button {
    label: Label,
    callback: Box<dyn FnMut()>,
}

impl Button {
    fn new(label: &str, callback: Box<dyn FnMut()>) -> Button {
        Button {
            label: Label::new(label),
            callback,
        }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }
}


impl Widget for Label {
    fn width(&self) -> usize {
        self.label.len() + 2 as usize
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        buffer.write_str(format!("{}\n", self.label).as_str()).unwrap();
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width() as usize
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let width = self.width();
        let line = String::from("-".repeat(width));
        buffer.write_str(format!("+{}+\n|{: ^width$}|\n+{}+\n", line, self.label.label, line).as_str()).unwrap();
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        std::cmp::max(
            self.title.len(),
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
        )
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let mut width = self.width();
        let line1 = String::from("-".repeat(width));
        let line2 = String::from("=".repeat(width));
        width -= 1;
        buffer.write_str(format!("+{}+\n| {: ^width$}|\n+{}+\n", line1, self.title.as_str(), line2).as_str()).unwrap();
        let mut widget_buffer = String::new();
        for widget in self.widgets.iter() {
            widget_buffer.write_char('\n').unwrap();
            widget.draw_into(&mut widget_buffer);
            widget_buffer.write_char('\n').unwrap();
        }
        for line in widget_buffer.lines() {
            buffer.write_str(format!("| {: <width$}|\n", line).as_str()).unwrap();
        }
        buffer.write_str(format!("+{}+", line1).as_str()).unwrap();
    }
}

fn main() {
    let mut window = Window::new("Rust GUI by Emivvvvv");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new(
        "Click me!",
        Box::new(|| println!("You clicked the button!")),
    )));
    window.draw();
}