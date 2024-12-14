use chartrs::{
    coord::{
        cartesian::{Cartesian2dY, HorizontalLine},
        ranged1d::KeyPointHint,
    },
    prelude::{DrawingArea, DrawingAreaErrorKind, Ranged},
};
use chartrs_backend::{BackendCoord, DrawingBackend, DrawingErrorKind};

pub(crate) trait LogPlotDrawingAreaFunc<Y, DB: DrawingBackend> {
    fn draw_plot_horizontal<DrawFunc, YH>(
        &self,
        y_keypoints: YH,
        draw_func: DrawFunc,
    ) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>>
    where
        YH: KeyPointHint,
        DrawFunc: FnMut(&mut DB, HorizontalLine<Y>, bool) -> Result<(), DrawingErrorKind<DB::ErrorType>>;
}

pub(crate) trait LogPlotCatesian2dYFunc<Y> {
    fn draw_plot_horizontal<E, YH, DrawFunc>(&self, y_keypoints: YH, draw_hori: DrawFunc) -> Result<(), E>
    where
        YH: KeyPointHint,
        DrawFunc: FnMut(HorizontalLine<Y>, bool) -> Result<(), E>;
}

impl<DB, Y> LogPlotDrawingAreaFunc<Y, DB> for DrawingArea<DB, Cartesian2dY<Y>>
where
    DB: DrawingBackend,
    Y: Ranged,
{
    fn draw_plot_horizontal<DrawFunc, YH>(
        &self,
        y_keypoints: YH,
        mut draw_func: DrawFunc,
    ) -> Result<(), DrawingAreaErrorKind<<DB as DrawingBackend>::ErrorType>>
    where
        YH: KeyPointHint,
        DrawFunc: FnMut(&mut DB, HorizontalLine<Y>, bool) -> Result<(), DrawingErrorKind<<DB as DrawingBackend>::ErrorType>>,
    {
        self.backend_ops(move |drawing_backend| {
            self.coord()
                .draw_plot_horizontal(y_keypoints, |line, is_dark| draw_func(drawing_backend, line, is_dark))
        })
    }
}

impl<Y> LogPlotCatesian2dYFunc<Y> for Cartesian2dY<Y>
where
    Y: Ranged,
{
    fn draw_plot_horizontal<E, YH, DrawFunc>(&self, y_keypoints: YH, mut draw_hori: DrawFunc) -> Result<(), E>
    where
        YH: KeyPointHint,
        DrawFunc: FnMut(HorizontalLine<Y>, bool) -> Result<(), E>,
    {
        let light_count = y_keypoints.max_points();
        let bold_count = y_keypoints.bold_points();

        let k = (light_count as f32 / bold_count as f32).round() as usize + 1;

        let ykp = self.logic_y().key_points(y_keypoints);

        let back_x = self.back_x();

        for (i, logic_y) in ykp.into_iter().enumerate() {
            let y = self.logic_y().map(&logic_y, self.back_y());

            draw_hori(
                HorizontalLine::new(BackendCoord::new(back_x.0, y), BackendCoord::new(back_x.1, y), &logic_y),
                i % k == 0,
            )?;
        }

        Ok(())
    }
}
