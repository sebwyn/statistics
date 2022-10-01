/*use std::rc::Rc;

use plotters::coord::Shift;
use plotters::prelude::*;
use regex::internal::Char;

struct Chart<'a, XU, YU> 
where
    XU: Ranged,
    YU: Ranged,
{
    ctx: Rc<ChartContext<'a, BitMapBackend<'a>, Cartesian2d<XU, YU>>>
}

impl<'a, XU, YU> Chart<'a, XU, YU>
where
    XU: Ranged,
    YU: Ranged,
{
    fn new<'a>(file: &str, size: (u32, u32), name: &str, x_range: XU, y_range: YU) -> Self {
        //let bmap: BitMapBackend<'a> = BitMapBackend::new(file, size);
        let bmap: BitMapBackend = BitMapBackend::new(file, size);
        let root: Box<'a, DrawingArea<BitMapBackend, Shift>> = Box::new(bmap.into_drawing_area());
        root.fill(&WHITE).unwrap(); //set a background color

        //construct our chart per specifications
        let chart: ChartContext<'a, BitMapBackend, Cartesian2d<XU, YU>> =
            ChartBuilder::on(root.as_ref())
                .caption(name, ("sans-serif", 50).into_font())
                .margin(5)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .build_cartesian_2d(x_range, y_range)
                .unwrap();

        Self { ctx: chart }
    }
}

*/
