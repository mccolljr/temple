use std::fmt::Formatter;

pub mod syntax;

pub type Result<T = ()> = std::result::Result<T, ()>;

pub trait Renderer: std::fmt::Write {}

impl<T> Renderer for T where T: std::fmt::Write {}

pub trait Renderable {
    fn render<R: Renderer>(&self, renderer: R) -> crate::Result;

    fn render_string(&self) -> crate::Result<String> {
        let mut output = String::new();
        self.render(&mut output)?;
        Ok(output)
    }
}

impl<T> Renderable for T
where
    T: std::fmt::Display,
{
    fn render<R: Renderer>(&self, mut renderer: R) -> crate::Result {
        match write!(renderer, "{}", self) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}

pub struct DynDisplayImpl<'a>(Box<dyn Fn(&mut std::fmt::Formatter) -> std::fmt::Result + 'a>);

impl<'a> std::fmt::Display for DynDisplayImpl<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0(f)
    }
}

pub trait AsDynDisplay {
    fn as_dyn_display(&self) -> DynDisplayImpl;
}

impl<T> AsDynDisplay for T
where
    T: Sized + Renderable,
{
    fn as_dyn_display(&self) -> DynDisplayImpl<'_> {
        DynDisplayImpl(Box::new(|f| match self.render(f) {
            Ok(_) => Ok(()),
            Err(_) => Err(std::fmt::Error),
        }))
    }
}

pub trait Template: Renderable {
    const TEMPLATE_DATA: &'static str;
}

#[cfg(test)]
mod tests;
