// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, Workbook, XlsxError};

// Test case to demonstrate creating a basic file with some string cell data.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let format1 = Format::new().set_bold();
    let format2 = Format::new().set_italic();

    let worksheet = workbook.add_worksheet();
    worksheet.write_string_with_format(0, 0, "Hello", &format1)?;
    worksheet.write_string_with_format(1, 0, "World", &format2)?;

    let worksheet = workbook.add_worksheet();
    worksheet.write_string_with_format(0, 0, "Hello", &format2)?;
    worksheet.write_string_with_format(1, 0, "World", &format1)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap18_duplicate_formats_in_different_order() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap18")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
