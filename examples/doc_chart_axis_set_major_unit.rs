// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

//! A chart example demonstrating setting the units for chart axes.

use rust_xlsxwriter::{Chart, ChartType, Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Add some data for the chart.
    worksheet.write(0, 0, 10)?;
    worksheet.write(1, 0, 30)?;
    worksheet.write(2, 0, 40)?;
    worksheet.write(3, 0, 30)?;
    worksheet.write(4, 0, 10)?;

    // Create a new chart.
    let mut chart = Chart::new(ChartType::Column);

    // Add a data series using Excel formula syntax to describe the range.
    chart.add_series().set_values("Sheet1!$A$1:$A$5");

    // Turn on the minor gridlines.
    chart.y_axis().set_minor_gridlines(true);

    // Set the value axes major and minor units.
    chart.y_axis().set_major_unit(20);
    chart.y_axis().set_minor_unit(5);

    // Hide legend for clarity.
    chart.legend().set_hidden();

    // Add the chart to the worksheet.
    worksheet.insert_chart(0, 2, &chart)?;

    // Save the file.
    workbook.save("chart.xlsx")?;

    Ok(())
}
