// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Chart, ChartFont, ChartType, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    worksheet.write(0, 0, "X")?;
    worksheet.write(1, 0, 1)?;
    worksheet.write(2, 0, 3)?;
    worksheet.write(0, 1, "Y")?;
    worksheet.write(1, 1, 10)?;
    worksheet.write(2, 1, 30)?;

    let mut chart = Chart::new(ChartType::Scatter);
    chart.set_axis_ids(82071936, 82074624);
    chart
        .add_series()
        .set_categories("=Sheet1!$A$2:$A$3")
        .set_values("=Sheet1!$B$2:$B$3");

    chart
        .x_axis()
        .set_name("=Sheet1!$A$1")
        .set_name_font(ChartFont::new().set_italic());

    chart.y_axis().set_name("=Sheet1!$B$1");

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_scatter15() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_scatter15")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
