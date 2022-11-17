// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxError};

mod common;

// Test to demonstrate simple hyperlinks.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let format = Format::new().set_hyperlink();

    worksheet.write_url(0, 0, "https://www.rust-lang.org/")?;
    worksheet.write_url_with_text(2, 0, "https://www.rust-lang.org/", "Rust")?;
    worksheet.write_url_with_format(4, 0, "https://www.rust-lang.org/", &format)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap49_hyperlinks() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap49")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
