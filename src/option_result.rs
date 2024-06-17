use std::fmt::Display;
use sailfish::RenderError;
use sailfish::runtime::{Buffer, Render};

pub enum Opt<T> {
    None,
    Some(T),
}

pub enum Res<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> From<Result<T, E>> for Res<T, E> {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(t) => Res::Ok(t),
            Err(e) => Res::Err(e),
        }
    }
}

impl<T> From<Option<T>> for Opt<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(t) => Opt::Some(t),
            None => Opt::None
        }
    }
}

impl<T: Render> Render for Opt<T> {
    #[inline]
    fn render(&self, b: &mut Buffer) -> Result<(), RenderError> {
        if let Opt::Some(inner) = self {
            inner.render(b)
        } else {
            Ok(())
        }
    }

    #[inline]
    fn render_escaped(&self, b: &mut Buffer) -> Result<(), RenderError> {
        if let Opt::Some(inner) = self {
            inner.render_escaped(b)
        } else {
            Ok(())
        }
    }
}

impl<T: Render, E: Render> Render for Res<T, E> {
    #[inline]
    fn render(&self, b: &mut Buffer) -> Result<(), RenderError> {
        match self {
            Res::Ok(inner) => { inner.render(b) }
            Res::Err(inner) => {
                "Error: ".render(b)?;
                inner.render(b)
            }
        }
    }

    #[inline]
    fn render_escaped(&self, b: &mut Buffer) -> Result<(), RenderError> {
        match self {
            Res::Ok(inner) => { inner.render_escaped(b) }
            Res::Err(inner) => {
                "Error: ".render(b)?;
                inner.render_escaped(b)
            }
        }
    }
}