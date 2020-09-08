use crate::error;
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum StatusValue {
    Infeasible(f64),
    Linear(f64),
    Integer(f64),
    Timeout(f64),
}

pub fn load_status_file(file_name: &Path) -> Result<Vec<StatusValue>, error::ProgramError> {
    let data = load_file_data(file_name)?;
    parse_log_file(&data)
}

fn parse_log_file(data: &str) -> Result<Vec<StatusValue>, error::ProgramError> {
    data.lines().map(parse_csv_line).collect()
}

fn load_file_data(file_name: &Path) -> Result<String, Error> {
    let mut file = File::open(file_name)?;
    let mut output = String::new();
    file.read_to_string(&mut output)?;
    Ok(output)
}

fn parse_csv_line(line: &str) -> Result<StatusValue, error::ProgramError> {
    let mut tokens = line.split(',');
    let size = convert_float_token(tokens.next(), 0)?;
    let status = convert_usize_token(tokens.next(), 1)?;

    let output = match status {
        Some(0) => StatusValue::Linear(size),
        Some(1) => StatusValue::Integer(size),
        Some(2) => StatusValue::Timeout(size),
        None => StatusValue::Infeasible(size),
        _ => {
            let err = error::ProgramError::UnknowStatus(status.unwrap());
            return Err(err);
        }
    };

    Ok(output)
}

fn convert_float_token(token: Option<&str>, col: usize) -> Result<f64, error::ParseError> {
    let token = get_token(token, col)?;

    Ok(token.parse()?)
}

fn convert_usize_token(
    token: Option<&str>,
    col: usize,
) -> Result<Option<usize>, error::ParseError> {
    let token = get_token(token, col)?;
    if token.len() > 0 {
        Ok(Some(token.parse()?))
    } else {
        Ok(None)
    }
}

fn get_token(token: Option<&str>, col: usize) -> Result<&str, error::ParseError> {
    if let Some(token) = token {
        Ok(token.trim())
    } else {
        Err(error::ParseError::MissingToken(col))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_log_load() {
        let data = r#"0.76,0
        0.56,1
        0.12,
        0.56,2
        0.12,2
        0.7678,0
        0.80,"#;

        let ans = parse_log_file(data).unwrap();
        let correct = vec![
            StatusValue::Linear(0.76),
            StatusValue::Integer(0.56),
            StatusValue::Infeasible(0.12),
            StatusValue::Timeout(0.56),
            StatusValue::Timeout(0.12),
            StatusValue::Linear(0.7678),
            StatusValue::Infeasible(0.8),
        ];

        assert_eq!(ans, correct);
    }
}
