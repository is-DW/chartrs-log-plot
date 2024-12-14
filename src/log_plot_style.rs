use std::marker::PhantomData;

use chartrs::{
    coord::{
        cartesian::Cartesian2dY,
        ranged1d::{LightPoints, ValueFormatter},
    },
    prelude::{DrawingAreaErrorKind, DrawingBackend, Ranged},
};
use chartrs_backend::{stroke::Stroke, FontDesc};

use crate::plot::log_plot_context::LogPlotContext;

type Fmt<'b, YV> = Option<&'b dyn Fn(&YV) -> String>;

pub struct LogPlotStyle<'a, 'b, Y, DB>
where
    Y: Ranged,
    DB: DrawingBackend,
{
    pub(super) parent_size: (u32, u32),

    pub(super) title: String,
    pub(super) detail: String,

    pub(super) y_label_format: Fmt<'b, Y::ValueType>,

    pub(super) chart_context: Option<&'b mut LogPlotContext<'a, DB, Cartesian2dY<Y>>>,
    pub(super) _phantom_data: PhantomData<Y>,
}

impl<'a, 'b, Y, YT, DB> LogPlotStyle<'a, 'b, Y, DB>
where
    DB: DrawingBackend,
    Y: Ranged<ValueType = YT> + ValueFormatter<YT>,
{
    pub(crate) fn new(chart_context: &'b mut LogPlotContext<'a, DB, Cartesian2dY<Y>>) -> Self {
        LogPlotStyle {
            parent_size: chart_context.depth_body.dim_in_pixel(),

            title: String::from("DEPTH CHANNEL"),
            detail: String::from("DEPTH"),

            y_label_format: None,

            chart_context: Some(chart_context),
            _phantom_data: PhantomData,
        }
    }
}

impl<'a, 'b, Y, DB> LogPlotStyle<'a, 'b, Y, DB>
where
    Y: Ranged,
    DB: DrawingBackend,
{
    pub fn title(&mut self, title: String, detail: String) -> &mut Self {
        self.title = title;
        self.detail = detail;

        self
    }

    /// Set the formatter function for the Y label text
    /// - `fmt`: The formatter function
    pub fn y_label_formatter(&mut self, fmt: &'b dyn Fn(&Y::ValueType) -> String) -> &mut Self {
        self.y_label_format = Some(fmt);
        self
    }

    pub fn draw(&mut self) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>>
    where
        Y: ValueFormatter<<Y as Ranged>::ValueType>,
    {
        let chart_context = if let Some(context) = self.chart_context.take() {
            context
        } else {
            return Err(DrawingAreaErrorKind::DrawingContextError);
        };

        // draw head
        chart_context.draw_head(self.title.as_str(), self.detail.as_str())?;
        // draw head & body outline
        chart_context.draw_area_rect(&Stroke::WIDTH2_BLACK_LINE)?;

        let text_style = &FontDesc::default_font_with_parent_size(&self.parent_size).into();

        let range = chart_context.depth_body.get_y_axis_pixel_range();
        let (start, end) = (range.start, range.end);

        let (mut light_count, mut bold_count) = (0, 0);

        for (i, _) in range.step_by(((end - start).abs() / (end / 40)) as usize).enumerate() {
            if i % 5 != 0 {
                light_count += 1;
            } else {
                bold_count += 1;
            }
        }

        // light & dark tick
        chart_context.draw_depth_area_tick(
            LightPoints::new(bold_count, light_count),
            &Stroke::LIGHT_MESH_LINE,
            &Stroke::BOLD_MESH_LINE,
            text_style,
            8,
            |y_range, m| {
                let label = self
                    .y_label_format
                    .map(|fmt_func| fmt_func(m.2))
                    .unwrap_or(y_range.format_ext(m.2));
                Some(label)
            },
        )?;

        Ok(())
    }
}
