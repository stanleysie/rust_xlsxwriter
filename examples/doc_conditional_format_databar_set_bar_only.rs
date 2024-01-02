// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

//! Example of adding a data bar type conditional formatting to a worksheet with
//! the bar only and with the data hidden.

use rust_xlsxwriter::{ConditionalFormatDataBar, Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    // Create a new Excel file object.
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Write the worksheet data.
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    worksheet.write_column(2, 1, data)?;
    worksheet.write_column(2, 3, data)?;

    // Write a standard Excel data bar.
    let conditional_format = ConditionalFormatDataBar::new();

    worksheet.add_conditional_format(2, 1, 11, 1, &conditional_format)?;

    // Write a data bar with the data hidden and bars shown only.
    let conditional_format = ConditionalFormatDataBar::new().set_bar_only(true);

    worksheet.add_conditional_format(2, 3, 11, 3, &conditional_format)?;

    // Save the file.
    workbook.save("conditional_format.xlsx")?;

    Ok(())
}
