// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

//! An example of adding markers to a line chart.

use rust_xlsxwriter::{
    Chart, ChartFormat, ChartMarker, ChartMarkerType, ChartSolidFill, ChartType, Workbook,
    XlsxError,
};

fn main() -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Add some data for the chart.
    worksheet.write(0, 0, 10)?;
    worksheet.write(1, 0, 40)?;
    worksheet.write(2, 0, 50)?;
    worksheet.write(3, 0, 20)?;
    worksheet.write(4, 0, 10)?;
    worksheet.write(5, 0, 50)?;

    // Create a new chart.
    let mut chart = Chart::new(ChartType::Line);

    // Add a data series with formatting.
    chart
        .add_series()
        .set_values("Sheet1!$A$1:$A$6")
        .set_marker(
            ChartMarker::new()
                .set_type(ChartMarkerType::Square)
                .set_size(10)
                .set_format(
                    ChartFormat::new().set_solid_fill(ChartSolidFill::new().set_color("#FF0000")),
                ),
        );

    // Add the chart to the worksheet.
    worksheet.insert_chart(0, 2, &chart)?;

    // Save the file.
    workbook.save("chart.xlsx")?;

    Ok(())
}
