// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{
    Chart, ChartDataLabel, ChartDataLabelPosition, ChartFont, ChartFormat, ChartLine, ChartType,
    Workbook, XlsxError,
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

    let data_labels = vec![ChartDataLabel::new()
        .set_font(
            ChartFont::new()
                .set_color("#FF0000")
                .set_bold()
                .set_italic(),
        )
        .set_format(ChartFormat::new().set_line(ChartLine::new().set_color("#FF0000")))
        .to_custom()];

    let mut chart = Chart::new(ChartType::Column);
    chart.set_axis_ids(84605184, 84639744);
    chart
        .add_series()
        .set_values(("Sheet1", 0, 0, 4, 0))
        .set_data_label(
            ChartDataLabel::new()
                .show_value()
                .set_position(ChartDataLabelPosition::Center),
        )
        .set_custom_data_labels(&data_labels);

    chart.add_series().set_values(("Sheet1", 0, 1, 4, 1));
    chart.add_series().set_values(("Sheet1", 0, 2, 4, 2));

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_data_labels50() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_data_labels50")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
