use chartrs::prelude::DynElement;
use chartrs_backend::{BackendCoord, DrawingBackend};

type SeriesAnnoDrawFn<'a, DB> = dyn Fn(BackendCoord) -> DynElement<'a, DB, BackendCoord> + 'a;

#[allow(dead_code)]
pub struct LogGraphSeries<'a, DB>
where
    DB: DrawingBackend,
{
    label: Option<String>,
    draw_func: Option<Box<SeriesAnnoDrawFn<'a, DB>>>,
}

impl<'a, DB: DrawingBackend> LogGraphSeries<'a, DB> {
    pub(crate) fn new() -> Self {
        Self {
            label: None,
            draw_func: None,
        }
    }
}
