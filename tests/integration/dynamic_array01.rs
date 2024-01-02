// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Formula, Workbook, XlsxError};

// Test case to test dynamic arrays/formulas.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write_dynamic_array_formula(0, 0, 0, 0, "=AVERAGE(TIMEVALUE(B1:B2))")?;
    worksheet.write_string(0, 1, "12:00")?;
    worksheet.write_string(1, 1, "12:00")?;

    workbook.save(filename)?;

    Ok(())
}

// Test case to test dynamic arrays/formulas with Formula.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write_dynamic_array_formula(
        0,
        0,
        0,
        0,
        Formula::new("=AVERAGE(TIMEVALUE(B1:B2))"),
    )?;
    worksheet.write_string(0, 1, "12:00")?;
    worksheet.write_string(1, 1, "12:00")?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_dynamic_array01() {
    let test_runner = common::TestRunner::new()
        .set_name("dynamic_array01")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_dynamic_array01_formula() {
    let test_runner = common::TestRunner::new()
        .set_name("dynamic_array01")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
