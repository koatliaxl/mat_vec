use super::Matrix4x4;
use std::fmt::{Display, Write};
use std::ops::{Add, AddAssign, Mul};

// .unwrap() on every write! because write to string can't produce error

impl<T> Matrix4x4<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + AddAssign + Default + Display,
{
    pub fn format_uniform_columns(&self) -> String {
        let mut buffers = Vec::new();
        let mut max_len = 0;
        for r in 0..4 {
            let mut row_buffers = Vec::new();
            for c in 0..4 {
                let s = self[(r, c)].to_string();
                let current_len = s.chars().count();
                if current_len > max_len {
                    max_len = current_len
                }
                row_buffers.push(s);
            }
            buffers.push(row_buffers);
        }
        let mut result = String::new();
        for r in 0..4 {
            result += "|";
            for c in 0..4 {
                write!(&mut result, " {:>1$}", buffers[r][c], max_len).unwrap();
                if c < 3 {
                    write!(&mut result, " ").unwrap();
                }
            }
            result += " |\n";
        }
        result
    }

    pub fn format_align_rows(&self) -> String {
        let mut buffers = Vec::new();
        let mut columns_max_lengths = [0; 4];
        for r in 0..4 {
            let mut row_buffers = Vec::new();
            for c in 0..4 {
                let s = self[(r, c)].to_string();
                //let s = format!("{:>7}", self[(r,c)]);
                let current_len = s.chars().count();
                if current_len > columns_max_lengths[c] {
                    columns_max_lengths[c] = current_len
                }
                row_buffers.push(s);
            }
            buffers.push(row_buffers);
        }
        let mut result = String::new();
        for r in 0..4 {
            result += "|";
            for c in 0..4 {
                write!(
                    &mut result,
                    " {:>1$}",
                    buffers[r][c], columns_max_lengths[c]
                )
                .unwrap();
                if c < 3 {
                    write!(&mut result, " ").unwrap();
                }
            }
            result += " |\n";
        }
        result
    }

    #[deprecated]
    pub fn simple_format(&self) -> String {
        let mut s = String::new();
        for r in 0..4 {
            writeln!(
                s,
                "| {0:}, {1:}, {2:}, {3:} |",
                self[(r, 0)],
                self[(r, 1)],
                self[(r, 2)],
                self[(r, 3)]
            )
            .unwrap();
        }
        s
    }
}

pub trait FractionalFormat {
    const FRACTION_DELIMITER: char;
}

impl FractionalFormat for f32 {
    const FRACTION_DELIMITER: char = '.';
}
impl FractionalFormat for f64 {
    const FRACTION_DELIMITER: char = '.';
}

impl<T> Matrix4x4<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + AddAssign + Default + Display + FractionalFormat,
{
    pub fn fmt_align_mag(&self) -> String {
        self.format_align_magnitudes()
    }

    pub fn fmt_with_prec(&self, precision: usize) -> String {
        self.format_with_precision(precision)
    }

    pub fn format_align_magnitudes(&self) -> String {
        self.format_fractional(None)
    }

    pub fn format_with_precision(&self, precision: usize) -> String {
        self.format_fractional(Some(precision))
    }

    fn format_fractional(&self, precision: Option<usize>) -> String {
        let mut buffers = Vec::new();
        let mut max_integer_lengths = [0; 4]; // per column
        let mut max_fraction_lengths = [0; 4]; // per column
        let mut sep_positions = [[None; 4]; 4];
        for r in 0..4 {
            let mut row_buffers = Vec::new();
            for c in 0..4 {
                let s = self[(r, c)].to_string();
                if let Some(delimiter) = s.find(T::FRACTION_DELIMITER) {
                    let (integer_part, fraction_part) = s.split_at(delimiter);
                    let integer_len = integer_part.chars().count();
                    let fraction_len = fraction_part.chars().count();
                    if integer_len > max_integer_lengths[c] {
                        max_integer_lengths[c] = integer_len
                    }
                    if fraction_len > max_fraction_lengths[c] {
                        max_fraction_lengths[c] = fraction_len;
                        if let Some(prc) = precision {
                            if max_fraction_lengths[c] > prc + 1 {
                                // +1 because fraction part includes delimiter
                                max_fraction_lengths[c] = prc + 1;
                            }
                        }
                    }
                    sep_positions[r][c] = Some(integer_len);
                } else {
                    let whole_len = s.chars().count();
                    if whole_len > max_integer_lengths[c] {
                        max_integer_lengths[c] = whole_len
                    }
                    sep_positions[r][c] = None;
                }
                row_buffers.push(s);
            }
            buffers.push(row_buffers);
        }
        let mut result = String::new();
        for r in 0..4 {
            result += "| ";
            for c in 0..4 {
                let (integer_part, fraction_part) = if let Some(sep_pos) = sep_positions[r][c] {
                    buffers[r][c].split_at(sep_pos)
                } else {
                    (buffers[r][c].as_str(), "")
                };
                write!(&mut result, "{:>1$}", integer_part, max_integer_lengths[c]).unwrap();
                match precision {
                    None => write!(
                        &mut result,
                        "{:1$} ",
                        fraction_part, max_fraction_lengths[c]
                    )
                    .unwrap(),
                    Some(precision) => write!(
                        &mut result,
                        "{:1$.2$} ",
                        fraction_part,
                        max_fraction_lengths[c],
                        precision + 1 // +1 because fraction part includes delimiter
                    )
                    .unwrap(),
                }
                if c < 3 {
                    write!(&mut result, " ").unwrap();
                }
            }
            result += "|\n";
        }
        result
    }
}
