use chartrs::{
    coord::{
        cartesian::{Cartesian2dY, MeshLine},
        ranged1d::{KeyPointHint, ValueFormatter},
    },
    prelude::{DrawingAreaErrorKind, DrawingBackend, Ranged},
};
use chartrs_backend::{stroke::Stroke, BackendCoord};

use crate::{
    mesh::ChannelContextMeshStyle,
    plot::channel_context::ChannelContext,
    plot_legend::{PlotCurve, PlotLegend},
};

impl<'a, DB, Y> ChannelContext<'a, DB, Cartesian2dY<Y>>
where
    DB: DrawingBackend,
    Y: Ranged + ValueFormatter<<Y as Ranged>::ValueType>,
{
    pub fn draw_head(
        &mut self,
        title: &str,
        legends: Vec<PlotCurve>,
    ) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>> {
        let (title_area, detail_area) = self.head.split_vertically(self.builder.title_height);

        // title
        let (x, y) = title_area
            .area_center_of_text(title, &self.builder.head_text_style)
            .unwrap_or((0, 0));

        title_area.draw_text(
            title,
            &self.builder.head_text_style,
            BackendCoord::new(x as i32, y as i32),
        )?;

        title_area.draw_outline(&Stroke::WIDTH2_BLACK_LINE)?;

        // legend
        let detail_text_style = &self.builder.detail_text_style;

        let legend = PlotLegend::with_area(
            &detail_area,
            legends,
            detail_text_style.clone(),
            detail_text_style.font_pct(0.8),
        );

        detail_area.draw(&legend)?;

        self.plot_legend = Some(legend);

        detail_area.draw_outline(&Stroke::WIDTH2_BLACK_LINE)?;

        Ok(())
    }

    pub fn draw_area_rect(
        &mut self,
        axis_style: &Stroke,
    ) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>> {
        self.head.draw_outline(axis_style)?;
        self.body.draw_outline(axis_style)
    }

    pub fn draw_mesh_lines<YH>(
        &mut self,
        x_keypoints: YH,
        y_keypoints: YH,
        mesh_style: &ChannelContextMeshStyle<Y, DB>,
        mesh_line_stroke: &Stroke,
    ) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>>
    where
        YH: KeyPointHint,
    {
        self.body
            .draw_mesh(y_keypoints, x_keypoints, |drawing_backend, mesh_line| {
                let draw = match mesh_line {
                    MeshLine::XMesh(_coord, _, _) => mesh_style.draw_x_mesh,
                    MeshLine::YMesh(_coord, _, _) => mesh_style.draw_y_mesh,
                };

                if draw {
                    mesh_line.draw(drawing_backend, mesh_line_stroke)
                } else {
                    Ok(())
                }
            })?;

        Ok(())
    }
}
