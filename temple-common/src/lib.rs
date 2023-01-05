pub mod parse;

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
        write!(renderer, "{}", self).map_err(|_| ())?;
        Ok(())
    }
}

pub trait Template: Renderable {
    const TEMPLATE_PATH: &'static str;
    const TEMPLATE_DATA: &'static str;
}

#[cfg(test)]
mod tests;
