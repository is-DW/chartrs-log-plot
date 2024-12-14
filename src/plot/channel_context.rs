use std::borrow::Borrow;

use crate::{builder::LogPlotBuilder, plot_legend::PlotLegend, series::LogGraphSeries};

use chartrs::{
    coord::{ranged1d::AsRangedCoord, CoordTranslate, Shift},
    element::{CoordMapper, Drawable, PointCollection},
    prelude::{Cartesian2d, DrawingArea, DrawingAreaErrorKind},
};
use chartrs_backend::{BackendCoord, DrawingBackend};

pub struct ChannelContext<'a, DB, CT>
where
    DB: DrawingBackend,
    CT: CoordTranslate,
{
    pub head: DrawingArea<DB, Shift>,
    pub body: DrawingArea<DB, CT>,

    pub series_anno: Vec<LogGraphSeries<'a, DB>>,

    pub plot_legend: Option<PlotLegend<'a, BackendCoord>>,

    pub(crate) builder: &'a LogPlotBuilder<'a, DB>,
}

impl<'a, DB, CT> ChannelContext<'a, DB, CT>
where
    DB: DrawingBackend,
    CT: CoordTranslate,
{
    /// Get a reference of underlying plotting area
    pub fn head_area(&self) -> &DrawingArea<DB, Shift> {
        &self.head
    }

    /// Get a reference of underlying plotting area
    pub fn body_area(&self) -> &DrawingArea<DB, CT> {
        &self.body
    }

    /// Cast the reference to a chart context to a reference to underlying coordinate specification.
    pub fn as_coord_spec(&self) -> &CT {
        self.body.as_coord_spec()
    }

    pub(crate) fn alloc_series_anno(&mut self) -> &mut LogGraphSeries<'a, DB> {
        let idx = self.series_anno.len();
        self.series_anno.push(LogGraphSeries::new());
        &mut self.series_anno[idx]
    }

    fn draw_series_impl<B, E, R, S>(
        area: &DrawingArea<DB, CT>,
        series: S,
    ) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>>
    where
        B: CoordMapper,
        for<'b> &'b E: PointCollection<'b, CT::From, B>,
        E: Drawable<DB, B>,
        R: Borrow<E>,
        S: IntoIterator<Item = R>,
    {
        for element in series {
            area.draw(element.borrow())?;
        }
        Ok(())
    }

    /// Draws a data series. A data series in Plotters is abstracted as an iterator of elements.
    ///
    /// See [`crate::series::LineSeries`] and [`ChartContext::configure_series_labels()`] for more information and examples.
    pub fn draw_series<B, E, R, S>(
        &mut self,
        series: S,
    ) -> Result<&mut LogGraphSeries<'a, DB>, DrawingAreaErrorKind<DB::ErrorType>>
    where
        B: CoordMapper,
        for<'b> &'b E: PointCollection<'b, CT::From, B>,
        E: Drawable<DB, B>,
        R: Borrow<E>,
        S: IntoIterator<Item = R>,
    {
        Self::draw_series_impl(&self.body, series)?;
        Ok(self.alloc_series_anno())
    }

    pub fn draw_series_with_range<Y, B, E, R, S>(
        &mut self,
        x_spec: Y,
        y_spec: Y,
        series: S,
    ) -> Result<&mut LogGraphSeries<'a, DB>, DrawingAreaErrorKind<DB::ErrorType>>
    where
        B: CoordMapper,
        for<'b> &'b E: PointCollection<
            'b,
            (
                <Y as AsRangedCoord>::ValueType,
                <Y as AsRangedCoord>::ValueType,
            ),
            B,
        >,
        E: Drawable<DB, B>,
        R: Borrow<E>,
        S: IntoIterator<Item = R>,
        Y: AsRangedCoord + Clone,
    {
        let r = self.body.get_pixel_range();

        let area = self
            .body
            .strip_coord_spec()
            .apply_coord_spec(Cartesian2d::<Y::CoordDescType, Y::CoordDescType>::new(
                x_spec, y_spec, r,
            ));

        for f in series {
            area.draw(f.borrow())?;
        }

        Ok(self.alloc_series_anno())
    }
}
