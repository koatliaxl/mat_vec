use super::Matrix4x4;
use std::fmt::{Display, Write};
use std::ops::{Add, AddAssign, Mul};

/*trait DisplayInteger {
}
trait DisplayFractional {
}*/
/*enum DisplayAlignment {
    //No,
    AlignRows,
    EqualColumnsWidth,
}*/

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
            result += "| ";
            for c in 0..4 {
                write!(&mut result, "{:>1$} ", buffers[r][c], max_len);
            }
            result += "|\n";
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
            result += "| ";
            for c in 0..4 {
                write!(
                    &mut result,
                    "{:>1$} ",
                    buffers[r][c], columns_max_lengths[c]
                );
            }
            result += "|\n";
        }
        result
    }
}
