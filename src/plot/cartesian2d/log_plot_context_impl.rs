use chartrs::{
    coord::{
        cartesian::{Cartesian2dY, HorizontalLine},
        ranged1d::KeyPointHint,
    },
    prelude::{DrawingAreaErrorKind, DrawingBackend, Ranged},
};
use chartrs_backend::{
    stroke::Stroke,
    text_anchor::{HPos, Pos, VPos},
    BackendCoord, TextStyle,
};

use crate::{cartesian_impl::LogPlotDrawingAreaFunc, plot::log_plot_context::LogPlotContext};

impl<'a, DB, Y> LogPlotContext<'a, DB, Cartesian2dY<Y>>
where
    DB: DrawingBackend,
    Y: Ranged,
{
    pub fn draw_head(&mut self, title: &str, detail: &str) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>> {
        let (title_area, detail_area) = self.depth_head.split_vertically(self.builder.title_height);

        // head
        let (x, y) = title_area
            .area_center_of_text(title, &self.builder.head_text_style)
            .unwrap_or((0, 0));

        title_area.draw_text(title, &self.builder.head_text_style, BackendCoord::new(x as i32, y as i32))?;

        title_area.draw_outline(&Stroke::WIDTH2_BLACK_LINE)?;

        // detail
        let (x, y) = detail_area
            .area_center_of_text(detail, &self.builder.detail_text_style)
            .unwrap_or((0, 0));

        detail_area.draw_text(detail, &self.builder.detail_text_style, BackendCoord::new(x as i32, y as i32))?;

        detail_area.draw_outline(&Stroke::WIDTH2_BLACK_LINE)?;

        Ok(())
    }

    pub fn draw_area_rect(&mut self, axis_style: &Stroke) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>> {
        self.depth_head.draw_outline(axis_style)?;
        self.depth_body.draw_outline(axis_style)
    }

    pub fn draw_depth_area_tick<FmtLabel, YH>(
        &mut self,
        y_keypoints: YH,
        light_axis_style: &Stroke,
        dark_axis_style: &Stroke,
        label_style: &TextStyle,
        tick_size: i32,
        fmt_label: FmtLabel,
    ) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>>
    where
        YH: KeyPointHint,
        FmtLabel: Fn(&Y, &HorizontalLine<Y>) -> Option<String>,
    {
        let label_dist = tick_size.abs();
        let (area_width, _) = self.depth_body.dim_in_pixel();
        let area_width = area_width as i32;

        let y_range = self.depth_body.as_coord_spec().y_spec();

        self.depth_body
            .draw_plot_horizontal(y_keypoints, |drawing_backend, mesh_line, is_dark| {
                let coord = mesh_line.0;

                // draw dark tick and label
                if is_dark {
                    let tick_size = tick_size + 5;

                    drawing_backend.draw_line(
                        BackendCoord::new(coord.x + area_width - tick_size, coord.y),
                        BackendCoord::new(coord.x + area_width, coord.y),
                        light_axis_style,
                    )?;

                    drawing_backend.draw_line(
                        BackendCoord::new(coord.x + area_width - tick_size, coord.y),
                        BackendCoord::new(coord.x + area_width, coord.y),
                        dark_axis_style,
                    )?;

                    if let Some(label_text) = fmt_label(y_range, &mesh_line) {
                        drawing_backend.draw_text(
                            label_text.as_str(),
                            &label_style.pos(Pos::new(HPos::Right, VPos::Center)),
                            BackendCoord::new(coord.x + area_width - tick_size - label_dist, coord.y),
                        )?;
                    }
                } else {
                    // draw light tick
                    drawing_backend.draw_line(
                        BackendCoord::new(coord.x + area_width - tick_size, coord.y),
                        BackendCoord::new(coord.x + area_width, coord.y),
                        light_axis_style,
                    )?;
                }

                Ok(())
            })?;

        Ok(())
    }
}
