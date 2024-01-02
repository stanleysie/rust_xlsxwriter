// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{FilterCondition, FilterCriteria, Workbook, XlsxError};

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
    let data = common::get_autofilter_data();
    for (row, data) in data.iter().enumerate() {
        let row = 1 + row as u32;
        worksheet.write_string(row, 0, data.0)?;
        worksheet.write_string(row, 1, data.1)?;
        worksheet.write_number(row, 2, data.2)?;
        worksheet.write_string(row, 3, data.3)?;
    }

    worksheet.autofilter(0, 0, 50, 3)?;

    let filter_condition1 = FilterCondition::new().add_list_filter("East");

    let filter_condition2 = FilterCondition::new()
        .add_custom_filter(FilterCriteria::GreaterThan, 3000)
        .add_custom_filter(FilterCriteria::LessThan, 8000);

    worksheet.filter_column(2, &filter_condition2)?;
    worksheet.filter_column(0, &filter_condition1)?;

    workbook.save(filename)?;

    Ok(())
}
#[test]
fn test_autofilter04() {
    let test_runner = common::TestRunner::new()
        .set_name("autofilter04")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
