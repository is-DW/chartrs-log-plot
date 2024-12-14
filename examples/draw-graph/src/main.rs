mod tmp;

use std::error::Error;

use chartrs::{
    prelude::{IntoDrawingArea, Stroke},
    series::LineSeries,
};
use chartrs_backend::{colors::*, palette::rgba::*, FontStyle};
use chartrs_bk::BkBackend;
use chartrs_log_plot::{builder::LogPlotBuilder, plot_legend::PlotCurve};

use tmp::{DEPTH, DTCO, GR_VALUE};

const OUT_FILE_NAME: &str = "output/graph.png";

fn main() -> Result<(), Box<dyn Error>> {
    let root = BkBackend::new(OUT_FILE_NAME, (550, 5000)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut builder = LogPlotBuilder::on(&root);

    let mut plot_context = builder
        .margin(10)
        .head_text_style(("Microsoft YaHei UI", 18, FontStyle::Bold))
        .detail_text_style(("Microsoft YaHei UI", 12, FontStyle::Bold))
        .add_depth_area(1524.0..1850.0);

    plot_context
        .configure_style()
        .title("DEPTH".to_string(), "DEPTH(m)".to_string())
        .draw()?;

    let mut channel_context = plot_context.add_channel(1524.0..1850.0);

    let mut c1 = PlotCurve::default();
    c1.name("GR")
        .range(30., 500.0, "gAPI")
        .stroke(Stroke::new(1.25, RED.to_backend_color()));

    let mut c2 = PlotCurve::default();
    c2.name("DTCO")
        .range(50., 150., "us/ft")
        .stroke(Stroke::new(1.25, BLUE.to_backend_color()));

    channel_context
        .configure_style()
        .title("CURVE".to_string())
        .draw(&[c1.clone(), c2.clone()])?;

    channel_context.draw_series_with_range(
        (c1.min as f64)..(c1.max as f64),
        1524.0..1850.0,
        LineSeries::new(
            GR_VALUE.iter().enumerate().map(|f| (*f.1, DEPTH[f.0])),
            c1.stroke,
        ),
    )?;

    channel_context.draw_series_with_range(
        (c2.min as f64)..(c2.max as f64),
        1524.0..1850.0,
        LineSeries::new(
            DTCO.iter().enumerate().map(|f| (*f.1, DEPTH[f.0])),
            c2.stroke,
        ),
    )?;

    let mut channel_context = plot_context.add_channel(1524.0..1850.0);

    let mut c1 = PlotCurve::default();
    c1.name("GR")
        .range(30., 500., "gAPI")
        .stroke(Stroke::new(1.25, RED.to_backend_color()));

    channel_context
        .configure_style()
        .title("CURVE".to_string())
        .draw(&[c1.clone()])?;

    channel_context.draw_series_with_range(
        (c1.min as f64)..(c1.max as f64),
        1524.0..1850.0,
        LineSeries::new(
            GR_VALUE.iter().enumerate().map(|f| (*f.1, DEPTH[f.0])),
            c1.stroke,
        ),
    )?;

    Ok(())
}
