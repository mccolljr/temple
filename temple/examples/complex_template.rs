use temple::{Renderable, Template};

#[derive(Template)]
#[template("temple/examples/templates/complex.tpl")]
struct Complex {
    items: Vec<Item>,
}

enum Item {
    A(String),
    B(i32),
    C(f32),
}

fn main() {
    println!(
        "{}",
        Complex {
            items: vec![Item::A("I am Item::A".into()), Item::B(12), Item::C(3.1415)]
        }
        .render_string()
        .unwrap()
    )
}
