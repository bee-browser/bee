use crate::style::*;
use crate::LayoutBox2D;
use crate::ToVisual;
use crate::VisualRenderer;

pub(crate) struct CanvasContainer {
    asset: Asset,
}

impl CanvasContainer {
    pub(crate) fn new(style: &Style) -> Self {
        CanvasContainer {
            asset: style.content.as_ref().unwrap().asset.clone(),
        }
    }

    pub(crate) fn inspect<W>(&self, write: &mut W, depth: usize) -> std::io::Result<()>
    where
        W: std::io::Write + ?Sized,
    {
        write!(write, "{:indent$}canvas:\n", "", indent = depth)?;
        Ok(())
    }

    pub(crate) fn render<R>(&self, renderer: &mut R, content_box: &LayoutBox2D)
    where
        R: VisualRenderer,
    {
        renderer.render_asset(self.asset.id, content_box.to_visual());
    }
}
