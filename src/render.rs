use std::path::Path;

mod render_svg;

use crate::diagram::TimingDiagram;

trait Renderer {
    type Error: std::error::Error;
    fn render(self, path: &Path) -> Result<(), Self::Error>;
}

pub fn render<P: AsRef<Path>>(diagram: &TimingDiagram, path: P) -> Result<(), anyhow::Error> {
    let path = path.as_ref();
    let renderer = match path.extension().and_then(std::ffi::OsStr::to_str) {
        Some("svg") => render_svg::RendererSvg::new(diagram),
        Some(invalid) => {
            return Err(anyhow::Error::msg(format!(
                "invalid output extension '{}'",
                invalid
            )))
        }
        None => return Err(anyhow::Error::msg("no output extension provided")),
    };
    renderer.render(path)?;
    Ok(())
}
