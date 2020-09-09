use plotters::prelude::*;

use crate::error;
use crate::log_file_loader::StatusValue;
use std::cmp::Ordering;
use std::path::Path;

pub struct Config {
    size: (u32, u32),
    margin: u32,
    radius: u32
}

impl Config {
    pub fn new() -> Self {
        Self {
            size: (0, 0),
            margin: 0,
            radius: 0
        }
    }

    pub fn set_size(mut self, width: u32, height: u32) -> Self {
        self.size = (width, height);
        self
    }

    pub fn set_margin(mut self, margin: u32) -> Self {
        self.margin = margin;
        self
    }

    pub fn set_radius(mut self, radius: u32) -> Self {
        self.radius = radius;
        self
    }

}

pub fn scatter_status(
    stat: &[StatusValue],
    name: &Path,
    config: Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let size = config.size;
    match Format::get_format(name)? {
        Format::PNG => generic_plotter(
            BitMapBackend::new(name, size).into_drawing_area(),
            stat,
            config,
        )?,
        Format::SVG => generic_plotter(
            SVGBackend::new(name, size).into_drawing_area(),
            stat,
            config,
        )?,
    }

    Ok(())
}

fn generic_plotter<'a, DB: DrawingBackend>(
    root: DrawingArea<DB, plotters::coord::Shift>,
    stat: &[StatusValue],
    config: Config,
) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>> {
    root.fill(&WHITE)?;
    let margin = config.margin;
    let root = root.margin(margin, margin, margin, margin);

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(20)
        .y_label_area_size(20)
        .build_cartesian_2d(
            0f32..(stat.len() as f32),
            0f32..((max_ratio(stat) * 1.1) as f32),
        )?;

    chart.configure_mesh().x_labels(5).y_labels(5).draw()?;

    chart.draw_series(stat.iter().enumerate().map(|(index, stat)| {
        let (y, color) = convert_status(stat);
        Circle::new((index as f32, y as f32), config.radius, color.filled())
    }))?;

    Ok(())
}

enum Format {
    PNG,
    SVG,
}

impl Format {
    fn get_format(name: &Path) -> Result<Self, error::FormatError> {
        if let Some(ext) = name.extension() {
            match ext.to_str() {
                Some("png") => Ok(Self::PNG),
                Some("svg") => Ok(Self::SVG),
                Some(other) => Err(error::FormatError::UnknownExtension(other.to_owned())),
                _ => panic!(),
            }
        } else {
            if let Some(name) = name.to_str() {
                Err(error::FormatError::MissingFormat(name.to_owned()))
            } else {
                panic!()
            }
        }
    }
}

fn convert_status(stat: &StatusValue) -> (f64, ShapeStyle) {
    match stat {
        StatusValue::Infeasible(v) => (*v, RED.filled()),
        StatusValue::Linear(v) => (*v, BLUE.filled()),
        StatusValue::Integer(v) => (*v, GREEN.filled()),
        StatusValue::Timeout(v) => (*v, BLACK.filled()),
    }
}

fn max_ratio(stat_vec: &[StatusValue]) -> f64 {
    stat_vec
        .iter()
        .map(|s| match s {
            StatusValue::Infeasible(s) => *s,
            StatusValue::Linear(s) => *s,
            StatusValue::Integer(s) => *s,
            StatusValue::Timeout(s) => *s,
        })
        .max_by(|a, b| {
            if a > b {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        })
        .unwrap()
}
