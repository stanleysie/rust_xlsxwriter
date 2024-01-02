// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{
    Chart, ChartErrorBars, ChartErrorBarsType, ChartLine, ChartMarker, ChartMarkerType, ChartType,
    ExcelDateTime, Format, Workbook, XlsxError,
};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(0, 11)?;
    worksheet.set_column_width(1, 11)?;
    worksheet.set_column_width(2, 11)?;
    worksheet.set_column_width(3, 11)?;

    let date_format = Format::new().set_num_format_index(14);

    // Add some test data for the chart(s).
    let dates = [
        ExcelDateTime::parse_from_str("2007-01-01")?,
        ExcelDateTime::parse_from_str("2007-01-02")?,
        ExcelDateTime::parse_from_str("2007-01-03")?,
        ExcelDateTime::parse_from_str("2007-01-04")?,
        ExcelDateTime::parse_from_str("2007-01-05")?,
    ];
    let high = [27.2, 25.03, 19.05, 20.34, 18.5];
    let low = [23.49, 19.55, 15.12, 17.84, 16.34];
    let close = [25.45, 23.05, 17.32, 20.45, 17.34];

    worksheet.write_column_with_format(0, 0, dates, &date_format)?;
    worksheet.write_column(0, 1, high)?;
    worksheet.write_column(0, 2, low)?;
    worksheet.write_column(0, 3, close)?;

    let mut chart = Chart::new(ChartType::Stock);
    chart.set_axis_ids(45470848, 45472768);

    chart
        .add_series()
        .set_categories(("Sheet1", 0, 0, 4, 0))
        .set_values(("Sheet1", 0, 1, 4, 1))
        .set_format(ChartLine::new().set_width(2.25).set_hidden(true))
        .set_marker(ChartMarker::new().set_none())
        .set_y_error_bars(ChartErrorBars::new().set_type(ChartErrorBarsType::StandardError));

    chart
        .add_series()
        .set_categories(("Sheet1", 0, 0, 4, 0))
        .set_values(("Sheet1", 0, 2, 4, 2))
        .set_format(ChartLine::new().set_width(2.25).set_hidden(true))
        .set_marker(ChartMarker::new().set_none())
        .set_y_error_bars(ChartErrorBars::new().set_type(ChartErrorBarsType::StandardError));
    chart
        .set_high_low_lines(true)
        .add_series()
        .set_categories(("Sheet1", 0, 0, 4, 0))
        .set_values(("Sheet1", 0, 3, 4, 3))
        .set_format(ChartLine::new().set_width(2.25).set_hidden(true))
        .set_marker(
            ChartMarker::new()
                .set_type(ChartMarkerType::ShortDash)
                .set_size(3),
        )
        .set_y_error_bars(ChartErrorBars::new().set_type(ChartErrorBarsType::StandardError));

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_errorbars07() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_errorbars07")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
