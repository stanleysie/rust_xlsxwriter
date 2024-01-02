// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Workbook, XlsxError};

// Test to demonstrate autofit.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write_string(0, 0, "Hello")?;
    worksheet.write_string(0, 1, "World")?;
    worksheet.write_number(0, 2, 123)?;
    worksheet.write_number(0, 3, 1234567)?;

    worksheet.autofit();

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_autofit04() {
    let test_runner = common::TestRunner::new()
        .set_name("autofit04")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
