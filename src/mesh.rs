use std::marker::PhantomData;

use chartrs::{
    coord::{
        cartesian::Cartesian2dY,
        ranged1d::{BoldPoints, LightPoints, ValueFormatter},
    },
    prelude::{DrawingAreaErrorKind, DrawingBackend, Ranged},
};
use chartrs_backend::stroke::Stroke;

use crate::{plot::channel_context::ChannelContext, plot_legend::PlotCurve};

pub struct ChannelContextMeshStyle<'a, 'b, Y, DB>
where
    Y: Ranged,
    DB: DrawingBackend,
{
    #[allow(dead_code)]
    pub(super) parent_size: (u32, u32),

    pub(super) dark_line_style: Option<Stroke>,
    pub(super) light_line_style: Option<Stroke>,

    pub(super) draw_x_mesh: bool,
    pub(super) draw_y_mesh: bool,

    pub(super) title: String,
    pub legends: Vec<PlotCurve>,

    pub(super) chart_context: Option<&'b mut ChannelContext<'a, DB, Cartesian2dY<Y>>>,
    pub(super) _phantom_data: PhantomData<Y>,
}

impl<'a, 'b, Y, YT, DB> ChannelContextMeshStyle<'a, 'b, Y, DB>
where
    DB: DrawingBackend,
    Y: Ranged<ValueType = YT> + ValueFormatter<YT>,
{
    pub(crate) fn new(chart_context: &'b mut ChannelContext<'a, DB, Cartesian2dY<Y>>) -> Self {
        ChannelContextMeshStyle {
            parent_size: chart_context.body.dim_in_pixel(),

            dark_line_style: None,
            light_line_style: None,

            draw_x_mesh: true,
            draw_y_mesh: true,

            title: String::default(),
            legends: vec![],

            chart_context: Some(chart_context),
            _phantom_data: PhantomData,
        }
    }
}

impl<'a, 'b, Y, DB> ChannelContextMeshStyle<'a, 'b, Y, DB>
where
    Y: Ranged,
    DB: DrawingBackend,
{
    pub fn title(&mut self, title: String) -> &mut Self {
        self.title = title;
        self
    }

    pub fn add_curve<F>(&mut self, mut desc: F) -> &mut Self
    where
        F: FnMut(&mut PlotCurve),
    {
        let mut curve = PlotCurve::default();

        desc(&mut curve);

        self.legends.push(curve);

        self
    }

    pub fn draw(&mut self, legends: &[PlotCurve]) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>>
    where
        Y: ValueFormatter<<Y as Ranged>::ValueType>,
    {
        let chart_context = if let Some(context) = self.chart_context.take() {
            context
        } else {
            return Err(DrawingAreaErrorKind::DrawingContextError);
        };
        // draw head
        chart_context.draw_head(self.title.as_str(), legends.to_owned())?;

        chart_context.draw_area_rect(&Stroke::WIDTH2_BLACK_LINE)?;

        let range = chart_context.body.get_y_axis_pixel_range();
        let (start, end) = (range.start, range.end);

        let (mut light_count, mut bold_count) = (0, 0);

        for (i, _) in range.step_by(((end - start).abs() / (end / 40)) as usize).enumerate() {
            if i % 5 != 0 {
                light_count += 1;
            } else {
                bold_count += 1;
            }
        }

        //----- light mesh -----
        chart_context.draw_mesh_lines(
            LightPoints::new(3, 3 * 4),
            LightPoints::new(bold_count, light_count),
            self,
            &self.light_line_style.unwrap_or(Stroke::LIGHT_MESH_LINE),
        )?;
        //----- bold mesh -----
        chart_context.draw_mesh_lines(
            BoldPoints(0),
            BoldPoints(bold_count),
            self,
            &self.dark_line_style.unwrap_or(Stroke::BOLD_MESH_LINE),
        )?;

        Ok(())
    }
}
