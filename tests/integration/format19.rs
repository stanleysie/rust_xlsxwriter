// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Color, Format, Workbook, XlsxError};

// Test to demonstrate number formats.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let format1 = Format::new().set_num_format("hh:mm;@");
    let format2 = Format::new()
        .set_num_format("hh:mm;@")
        .set_background_color(Color::Yellow);

    worksheet.write_with_format(0, 0, 1, &format1)?;
    worksheet.write_with_format(1, 0, 2, &format2)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_format19() {
    let test_runner = common::TestRunner::new()
        .set_name("format19")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
