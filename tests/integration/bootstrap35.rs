// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, Formula, Workbook, XlsxError};

// Test case to demonstrate creating a basic file with some string cell data.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let format1 = Format::new().set_bold();

    worksheet.write_formula(0, 0, "=1+2+3-6")?;
    worksheet.write_formula(1, 0, "=SIN(0)")?;
    worksheet.write_formula_with_format(2, 0, "SIN(0)", &format1)?; // No equals sign.
    worksheet
        .write_formula(3, 0, "=1+1")?
        .set_formula_result(3, 0, "2");
    worksheet
        .write_formula_with_format(4, 0, "1+1", &format1)?
        .set_formula_result(4, 0, "2");

    let worksheet = workbook.add_worksheet();
    worksheet.set_formula_result_default("2");
    worksheet.write_formula(0, 0, "=2")?;
    worksheet.write_formula(1, 0, "=1+1")?;

    workbook.save(filename)?;

    Ok(())
}

// Test case to demonstrate generic formula parameters.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let format1 = Format::new().set_bold();

    worksheet.write(0, 0, Formula::new("=1+2+3-6"))?;
    worksheet.write(1, 0, Formula::new("=SIN(0)".to_string()))?; // Uses String type.
    worksheet.write_with_format(2, 0, Formula::new("SIN(0)"), &format1)?; // No equals sign.
    worksheet.write(3, 0, Formula::new("=1+1").set_result("2"))?;
    worksheet.write_with_format(4, 0, Formula::new("1+1").set_result("2"), &format1)?;

    let worksheet = workbook.add_worksheet();
    worksheet.set_formula_result_default("2");
    worksheet.write(0, 0, Formula::new("=2"))?;
    worksheet.write(1, 0, Formula::new("=1+1"))?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap35_write_formula() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap35")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .ignore_calc_chain()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn bootstrap35_write_formula_generic() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap35")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .ignore_calc_chain()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
