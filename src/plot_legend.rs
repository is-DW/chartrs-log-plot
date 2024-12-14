use chartrs::{
    coord::Shift,
    element::{BackendCoordOnly, CoordMapper, Drawable, PointCollection},
    prelude::{DrawingArea, Stroke},
};
use chartrs_backend::{
    BackendCoord, BackendTextStyle, DrawingBackend, DrawingErrorKind, TextStyle,
};

#[derive(Debug, Clone, Default)]
pub struct PlotCurve {
    pub name: String,
    pub min: f32,
    pub max: f32,
    pub unit: String,
    pub stroke: Stroke,
}

impl PlotCurve {
    pub fn name<I>(&mut self, name: I) -> &mut Self
    where
        I: Into<String>,
    {
        self.name = name.into();
        self
    }

    pub fn range<I>(&mut self, min: f32, max: f32, unit: I) -> &mut Self
    where
        I: Into<String>,
    {
        self.min = min;
        self.max = max;
        self.unit = unit.into();
        self
    }

    pub fn stroke(&mut self, stroke: Stroke) -> &mut Self {
        self.stroke = stroke;
        self
    }
}

pub struct PlotLegend<'a, Coord> {
    points: [Coord; 2],
    legends: Vec<PlotCurve>,
    line_up: TextStyle<'a>,
    line_down: TextStyle<'a>,
}

impl<'a> PlotLegend<'a, BackendCoord> {
    pub fn with_area<DB>(
        area: &DrawingArea<DB, Shift>,
        legends: Vec<PlotCurve>,
        detail_top_style: TextStyle<'a>,
        detail_bottom_style: TextStyle<'a>,
    ) -> Self
    where
        DB: DrawingBackend,
    {
        let (w, h) = area.dim_in_pixel();

        Self {
            points: [
                BackendCoord::new(0_i32, 0_i32),
                BackendCoord::new(w as i32, h as i32),
            ],
            legends,
            line_up: detail_top_style,
            line_down: detail_bottom_style,
        }
    }
}

impl<'a, Coord> PlotLegend<'a, Coord> {
    pub fn new(
        points: [Coord; 2],
        legends: Vec<PlotCurve>,
        detail_top_style: TextStyle<'a>,
        detail_bottom_style: TextStyle<'a>,
    ) -> Self {
        Self {
            points,
            legends,
            line_up: detail_top_style,
            line_down: detail_bottom_style,
        }
    }
}

impl<'a, Coord> PointCollection<'a, Coord> for &'a PlotLegend<'a, Coord> {
    type Point = &'a Coord;
    type IntoIter = &'a [Coord];

    fn point_iter(self) -> &'a [Coord] {
        &self.points
    }
}

impl<'a, Coord, DB> Drawable<DB> for PlotLegend<'a, Coord>
where
    DB: DrawingBackend,
{
    fn draw<I>(
        &self,
        mut points: I,
        backend: &mut DB,
        parent_dim: (u32, u32),
    ) -> Result<(), DrawingErrorKind<<DB as DrawingBackend>::ErrorType>>
    where
        I: Iterator<Item = <BackendCoordOnly as CoordMapper>::Output>,
    {
        let count = self.legends.len() as i32;

        let stroke_height: f32 = self.legends.iter().map(|f| f.stroke.width).sum();

        let text_height = (self.line_up.size() + self.line_down.size() + 7.) as f32;
        let spacing = text_height as i32;

        let text_height = text_height * count as f32;
        let total_height = (stroke_height + text_height + (spacing * (count - 1)) as f32) as i32;

        let (real_width, real_height) = (parent_dim.0 as i32, parent_dim.1 as i32);

        match (points.next(), points.next()) {
            (Some(top_left), Some(bottom_right)) => {
                if real_height < total_height {
                    // todo
                    panic!()
                }

                let mut start_y = top_left.y + (real_height - total_height) / 2;

                for legend in self.legends.iter() {
                    let (_, (max_x, max_y)) = self
                        .line_up
                        .layout_box(&legend.name)
                        .map_err(|e| DrawingErrorKind::FontError(Box::new(e)))?;

                    backend.draw_text(
                        &legend.name,
                        &self.line_up,
                        BackendCoord::new(top_left.x + (real_width / 2 - max_x / 2), start_y),
                    )?;

                    start_y += max_y + 5;

                    backend.draw_line(
                        BackendCoord::new(top_left.x, start_y),
                        BackendCoord::new(bottom_right.x, start_y),
                        &legend.stroke,
                    )?;

                    start_y += 2;

                    backend.draw_text(
                        &legend.min.to_string(),
                        &self.line_down,
                        BackendCoord::new(top_left.x + 5, start_y),
                    )?;

                    // unit
                    let (_, (max_x, _max_y)) = self
                        .line_down
                        .layout_box(&legend.unit)
                        .map_err(|e| DrawingErrorKind::FontError(Box::new(e)))?;
                    backend.draw_text(
                        &legend.unit,
                        &self.line_down,
                        BackendCoord::new(top_left.x + real_width / 2 - max_x / 2, start_y),
                    )?;

                    // max
                    let (_, (max_x, _max_y)) =
                        self.line_down
                            .layout_box(&legend.max.to_string())
                            .map_err(|e| DrawingErrorKind::FontError(Box::new(e)))?;
                    backend.draw_text(
                        &legend.max.to_string(),
                        &self.line_down,
                        BackendCoord::new(bottom_right.x - max_x - 5, start_y),
                    )?;

                    start_y += spacing;
                }

                Ok(())
            }
            _ => Ok(()),
        }
    }
}
