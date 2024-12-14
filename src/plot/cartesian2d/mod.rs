use chartrs::{
    coord::{cartesian::Cartesian2dY, ranged1d::ValueFormatter},
    prelude::Ranged,
};
use chartrs_backend::DrawingBackend;

use crate::{
    log_plot_style::LogPlotStyle, mesh::ChannelContextMeshStyle,
    plot::channel_context::ChannelContext,
};

use crate::plot::log_plot_context::LogPlotContext;

pub(crate) mod channel_context_impl;
pub(crate) mod log_plot_context_impl;

impl<'a, DB, YT, Y> LogPlotContext<'a, DB, Cartesian2dY<Y>>
where
    DB: DrawingBackend,
    Y: Ranged<ValueType = YT> + ValueFormatter<YT>,
{
    /// Initialize a mesh configuration object and mesh drawing can be finalized by calling
    /// the function `MeshStyle::draw`.
    pub fn configure_style(&mut self) -> LogPlotStyle<'a, '_, Y, DB> {
        LogPlotStyle::new(self)
    }
}

impl<'a, DB, YT, Y> ChannelContext<'a, DB, Cartesian2dY<Y>>
where
    DB: DrawingBackend,
    Y: Ranged<ValueType = YT> + ValueFormatter<YT>,
{
    /// Initialize a mesh configuration object and mesh drawing can be finalized by calling
    /// the function `MeshStyle::draw`.
    pub fn configure_style(&mut self) -> ChannelContextMeshStyle<'a, '_, Y, DB> {
        ChannelContextMeshStyle::new(self)
    }
}
