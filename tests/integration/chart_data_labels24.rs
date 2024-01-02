// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{
    Chart, ChartDataLabel, ChartDataLabelPosition, ChartFont, ChartType, Workbook, XlsxError,
};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    let data = [[1, 2, 3], [2, 4, 6], [3, 6, 9], [4, 8, 12], [5, 10, 15]];
    for (row_num, row_data) in data.iter().enumerate() {
        for (col_num, col_data) in row_data.iter().enumerate() {
            worksheet.write_number(row_num as u32, col_num as u16, *col_data)?;
        }
    }

    let mut chart = Chart::new(ChartType::Column);
    chart.set_axis_ids(45937792, 45939712);
    chart
        .add_series()
        .set_values(("Sheet1", 0, 0, 4, 0))
        .set_data_label(
            ChartDataLabel::new().show_value().set_font(
                ChartFont::new()
                    .set_name("Consolas")
                    .set_size(12)
                    .set_pitch_family(49),
            ),
        );

    chart
        .add_series()
        .set_values(("Sheet1", 0, 1, 4, 1))
        .set_data_label(
            ChartDataLabel::new()
                .show_value()
                .set_position(ChartDataLabelPosition::InsideBase),
        );

    chart.add_series().set_values(("Sheet1", 0, 2, 4, 2));

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_data_labels24() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_data_labels24")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
