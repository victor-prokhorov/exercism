#![warn(clippy::pedantic)]

use std::str::from_utf8;

#[must_use]
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = vec![];

    let height = minefield.len();

    if !minefield.is_empty() {
        let width = minefield[0].as_bytes().len();

        let mut buffer: Vec<Vec<u8>> = vec![vec![b' '; width]; height];
        println!("width: {width:?} height: {height:?}");

        for (row_index, &str) in minefield.iter().enumerate() {
            for (column_index, &byte) in str.as_bytes().iter().enumerate() {
                println!("row_index: {row_index:?} column_index: {column_index:?}");

                if byte == b'*' {
                    buffer[row_index][column_index] = byte;

                    let possible_row_range: Vec<usize> = (row_index.saturating_sub(1)
                        ..=row_index.saturating_add(1))
                        .filter(|&row_index| row_index < height)
                        .collect();
                    let possible_column_range: Vec<usize> = (column_index.saturating_sub(1)
                        ..=column_index.saturating_add(1))
                        .filter(|&column_index| column_index < width)
                        .collect();
                    println!("possible_row_range: {possible_row_range:?} possible_column_range: {possible_column_range:?}");

                    for &delta_row_index in &possible_row_range {
                        for &delta_column_index in &possible_column_range {
                            println!("delta_row_index: {delta_row_index:?} delta_column_index: {delta_column_index:?}");

                            let byte = buffer[delta_row_index][delta_column_index];
                            println!("byte: {byte}");

                            if byte == b' ' {
                                buffer[delta_row_index][delta_column_index] = b'1';
                            } else if byte != b'*' {
                                buffer[delta_row_index][delta_column_index] = byte + 1;
                            }
                        }
                    }
                }
            }
        }

        for row in &buffer {
            if let Ok(row) = from_utf8(row) {
                result.push(row.to_string());
            }
        }
    }

    result
}
