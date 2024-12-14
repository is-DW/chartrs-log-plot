use std::fmt::Debug;

use chartrs::{
    coord::{cartesian::Cartesian2dY, ranged1d::AsRangedCoord, CoordTranslate, Shift},
    prelude::{DrawingArea, DrawingBackend},
};

use crate::{builder::LogPlotBuilder, plot::channel_context::ChannelContext};

pub struct LogPlotContext<'a, DB, CT>
where
    DB: DrawingBackend,
    CT: CoordTranslate,
{
    pub(crate) depth_head: DrawingArea<DB, Shift>,
    pub(crate) depth_body: DrawingArea<DB, CT>,

    pub(crate) drawing_area: DrawingArea<DB, Shift>,

    pub(crate) builder: &'a LogPlotBuilder<'a, DB>,
}

impl<'a, DB, CT> LogPlotContext<'a, DB, CT>
where
    DB: DrawingBackend,
    CT: CoordTranslate,
{
    pub fn add_channel<Y>(
        &mut self,
        y_spec: Y,
    ) -> ChannelContext<'a, DB, Cartesian2dY<Y::CoordDescType>>
    where
        Y: AsRangedCoord + Clone + Debug,
    {
        let (left, right) = self.drawing_area.split_horizontally(200);
        self.drawing_area = right;
        let (head, body) = left.split_vertically(self.builder.head_height);

        ChannelContext {
            head,
            body: body.apply_coord_spec(Cartesian2dY::new(y_spec.clone(), body.get_pixel_range())),
            series_anno: vec![],
            builder: self.builder,
            plot_legend: None,
        }
    }
}
