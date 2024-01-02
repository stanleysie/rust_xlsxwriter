// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{FilterCondition, Workbook, XlsxError};

// Test to demonstrate autofilters.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Write the headers.
    worksheet.write_string(0, 0, "Region")?;
    worksheet.write_string(0, 1, "Item")?;
    worksheet.write_string(0, 2, "Volume")?;
    worksheet.write_string(0, 3, "Month")?;

    // Write the data used in the autofilter.
    let mut data = common::get_autofilter_data();

    // Create a blank cell for testing.
    data[5].0 = "";

    for (row, data) in data.iter().enumerate() {
        let row = 1 + row as u32;
        worksheet.write_string(row, 0, data.0)?;
        worksheet.write_string(row, 1, data.1)?;
        worksheet.write_number(row, 2, data.2)?;
        worksheet.write_string(row, 3, data.3)?;
    }

    worksheet.autofilter(0, 0, 50, 3)?;

    let filter_condition = FilterCondition::new()
        .add_list_filter("North")
        .add_list_blanks_filter();

    worksheet.filter_column(0, &filter_condition)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_autofilter08() {
    let test_runner = common::TestRunner::new()
        .set_name("autofilter08")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
