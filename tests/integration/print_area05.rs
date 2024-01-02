// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Workbook, XlsxError};

// Test the creation of a simple rust_xlsxwriter file with a print area.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    worksheet.write_string(0, 0, "Foo")?;

    worksheet.set_print_area(0, 0, 1_048_575, 0)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_print_area05() {
    let test_runner = common::TestRunner::new()
        .set_name("print_area05")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
