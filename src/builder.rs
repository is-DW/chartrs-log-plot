use chartrs::{
    coord::{cartesian::Cartesian2dY, ranged1d::AsRangedCoord, Shift},
    prelude::DrawingArea,
};
use chartrs_backend::{DrawingBackend, IntoFont, SizeDesc, TextStyle};

use crate::plot::log_plot_context::LogPlotContext;

pub struct LogPlotBuilder<'a, DB>
where
    DB: DrawingBackend,
{
    pub(crate) root_area: &'a DrawingArea<DB, Shift>,
    pub(crate) margin: [u32; 4],
    pub(crate) depth_area_width: i32,
    pub(crate) head_height: i32,
    pub(crate) title_height: i32,

    pub(crate) head_text_style: TextStyle<'a>,
    pub(crate) detail_text_style: TextStyle<'a>,
}

impl<'a, DB> LogPlotBuilder<'a, DB>
where
    DB: DrawingBackend,
{
    pub fn on(root: &'a DrawingArea<DB, Shift>) -> Self {
        Self {
            root_area: root,
            margin: [0; 4],
            depth_area_width: 100,
            head_height: 200,
            title_height: 70,

            head_text_style: TextStyle::from(("sans-serif", 16).into_font()).hv_center(),
            detail_text_style: TextStyle::from(("sans-serif", 14).into_font()).hv_center(),
        }
    }

    pub fn head_text_style<S>(&mut self, head_text_style: S) -> &mut Self
    where
        S: Into<TextStyle<'a>>,
    {
        self.head_text_style = head_text_style.into();
        self
    }

    pub fn detail_text_style<S>(&mut self, detail_text_style: S) -> &mut Self
    where
        S: Into<TextStyle<'a>>,
    {
        self.detail_text_style = detail_text_style.into();
        self
    }

    pub fn margin<S: SizeDesc>(&mut self, size: S) -> &mut Self {
        let size = size.in_pixels(self.root_area).max(0) as u32;
        self.margin = [size, size, size, size];
        self
    }

    pub fn margin_top<S: SizeDesc>(&mut self, size: S) -> &mut Self {
        let size = size.in_pixels(self.root_area).max(0) as u32;
        self.margin[0] = size;
        self
    }

    pub fn margin_bottom<S: SizeDesc>(&mut self, size: S) -> &mut Self {
        let size = size.in_pixels(self.root_area).max(0) as u32;
        self.margin[1] = size;
        self
    }

    pub fn margin_left<S: SizeDesc>(&mut self, size: S) -> &mut Self {
        let size = size.in_pixels(self.root_area).max(0) as u32;
        self.margin[2] = size;
        self
    }

    pub fn margin_right<S: SizeDesc>(&mut self, size: S) -> &mut Self {
        let size = size.in_pixels(self.root_area).max(0) as u32;
        self.margin[3] = size;
        self
    }

    pub fn add_depth_area<Y>(
        &mut self,
        // x_spec: X,
        y_spec: Y,
    ) -> LogPlotContext<DB, Cartesian2dY<Y::CoordDescType>>
    where
        // X: AsRangedCoord + Clone,
        Y: AsRangedCoord + Clone,
    {
        // Now the root drawing area is to be split into
        //
        // +----------+------------------------------+
        // |   Depth  |    1 (Head Legend Area)      |
        // +----------+------------------------------+
        // |          |                              |
        // |          |                              |
        // |          |     4 (Plotting Area)        |
        // |          |                              |
        // |          |                              |
        // +----------+------------------------------+
        let (left, right) = self
            .root_area
            .apply_margin(self.margin)
            .split_horizontally(self.depth_area_width);

        let (left_head, left_body) = left.split_vertically(self.head_height);

        LogPlotContext {
            depth_head: left_head,
            depth_body: left_body.apply_coord_spec(Cartesian2dY::new(
                y_spec.clone(),
                left_body.get_pixel_range(),
            )),
            drawing_area: right,
            builder: self,
        }
    }
}
