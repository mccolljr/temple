use temple::{Renderable, Template};

#[derive(Template)]
#[template("temple/examples/templates/sub.tpl")]
struct SubTemplate {
    name: String,
}

#[derive(Template)]
#[template("temple/examples/templates/root.tpl")]
struct RootTemplate {
    name: String,
    sub_templates: Vec<SubTemplate>,
}

#[derive(Template, Clone, Copy)]
#[template("temple/examples/templates/enum.tpl")]
enum ThisWorksToo {
    A,
    B,
    C,
    D,
}

fn main() {
    println!(
        "{}",
        RootTemplate {
            name: "Root".into(),
            sub_templates: vec![
                SubTemplate { name: "A".into() },
                SubTemplate { name: "B".into() },
                SubTemplate { name: "C".into() },
                SubTemplate { name: "D".into() },
            ],
        }
        .render_string()
        .unwrap()
    );

    println!("{}", ThisWorksToo::A.render_string().unwrap());
    println!("{}", ThisWorksToo::B.render_string().unwrap());
    println!("{}", ThisWorksToo::C.render_string().unwrap());
    println!("{}", ThisWorksToo::D.render_string().unwrap());
}
