use geo_types::*;
use gnuplot::{Figure, PlotOption};

trait Plot {
    fn plot(&self, fg: &mut Figure, opt: &[PlotOption<&str>]);
}

impl Plot for Line<f64> {
    fn plot(&self, fg: &mut Figure, opt: &[PlotOption<&str>]) {
        let (sx, sy) = self.start.x_y();
        let (ex, ey) = self.end.x_y();
        let x = vec![sx, ex];
        let y = vec![sy, ey];
        fg.axes2d().lines(&x, &y, opt);
    }
}

impl Plot for LineString<f64> {
    fn plot(&self, fg: &mut Figure, opt: &[PlotOption<&str>]) {
        let (x, y): (Vec<f64>, Vec<f64>) = self.points_iter().map(|p| p.x_y()).unzip();
        fg.axes2d().lines(&x, &y, opt);
    }
}

impl Plot for Coordinate<f64> {
    fn plot(&self, fg: &mut Figure, opt: &[PlotOption<&str>]) {
        let (x, y) = self.x_y();
        fg.axes2d().points(vec![x], vec![y], opt);
    }
}

impl Plot for Point<f64> {
    fn plot(&self, fg: &mut Figure, opt: &[PlotOption<&str>]) {
        let (x, y) = self.x_y();
        fg.axes2d().points(vec![x], vec![y], opt);
    }
}

impl Plot for Polygon<f64> {
    fn plot(&self, fg: &mut Figure, opt: &[PlotOption<&str>]) {
        let (x, y): (Vec<f64>, Vec<f64>) = self.exterior().points_iter().map(|p| p.x_y()).unzip();
        fg.axes2d().lines(&x, &y, opt);
    }
}

impl Plot for MultiLineString<f64> {
    fn plot(&self, fg: &mut Figure, opt: &[PlotOption<&str>]) {
        self.clone().into_iter().for_each(|line| line.plot(fg, opt))
    }
}

impl Plot for MultiPolygon<f64> {
    fn plot(&self, fg: &mut Figure, opt: &[PlotOption<&str>]) {
        self.clone().into_iter().for_each(|poly| poly.plot(fg, opt))
    }
}

impl Plot for MultiPoint<f64> {
    fn plot(&self, fg: &mut Figure, opt: &[PlotOption<&str>]) {
        self.clone()
            .into_iter()
            .for_each(|point| point.plot(fg, opt))
    }
}

impl Plot for Geometry<f64> {
    fn plot(&self, fg: &mut Figure, opt: &[PlotOption<&str>]) {
        match self {
            Geometry::Point(x) => x.plot(fg, opt),
            Geometry::Line(x) => x.plot(fg, opt),
            Geometry::LineString(x) => x.plot(fg, opt),
            Geometry::Polygon(x) => x.plot(fg, opt),
            Geometry::MultiPoint(x) => x.plot(fg, opt),
            Geometry::MultiLineString(x) => x.plot(fg, opt),
            Geometry::MultiPolygon(x) => x.plot(fg, opt),
            Geometry::GeometryCollection(x) => x.plot(fg, opt),
        }
    }
}

impl Plot for GeometryCollection<f64> {
    fn plot(&self, fg: &mut Figure, opt: &[PlotOption<&str>]) {
        self.clone().into_iter().for_each(|g| g.plot(fg, opt))
    }
}
