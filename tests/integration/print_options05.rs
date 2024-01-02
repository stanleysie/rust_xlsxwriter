// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Workbook, XlsxError};

// Test to demonstrate various print options.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.set_print_gridlines(true);
    worksheet.set_print_center_horizontally(true);
    worksheet.set_print_center_vertically(true);
    worksheet.set_print_headings(true);

    worksheet.write_string(0, 0, "Foo")?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_print_options05() {
    let test_runner = common::TestRunner::new()
        .set_name("print_options05")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
