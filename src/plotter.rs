use plotters::prelude::*;

use crate::error;
use crate::log_file_loader::StatusValue;
use std::cmp::Ordering;
use std::path::Path;

pub fn scatter_status(stat: &[StatusValue], name: &Path) -> Result<(), Box<dyn std::error::Error>> {
    match Format::get_format(name)? {
        Format::PNG => generic_plotter(
            BitMapBackend::new(name, (640, 480)).into_drawing_area(),
            stat,
        )?,
        Format::SVG => {
            generic_plotter(SVGBackend::new(name, (640, 480)).into_drawing_area(), stat)?
        }
    }

    Ok(())
}

fn generic_plotter<'a, DB: DrawingBackend>(
    root: DrawingArea<DB, plotters::coord::Shift>,
    stat: &[StatusValue],
) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>> {
    root.fill(&WHITE)?;
    let root = root.margin(15, 15, 15, 15);

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(20)
        .y_label_area_size(40)
        .build_cartesian_2d(
            0f32..(stat.len() as f32),
            0f32..((max_ratio(stat) * 2.0) as f32),
        )?;

    chart.configure_mesh().x_labels(5).y_labels(5).draw()?;

    chart.draw_series(stat.iter().enumerate().map(|(index, stat)| {
        let (y, color) = convert_status(stat);
        Circle::new((index as f32, y as f32), 2, color.filled())
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
