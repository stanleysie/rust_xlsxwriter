// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxError};

mod common;

// Test case to demonstrate cell font charset formatting. This is a special case
// test for an Arabic font that requires the charset to be enabled/set for it to
// render properly.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new(filename);

    let format1 = Format::new()
        .set_font_name("B Nazanin")
        .set_font_family(0)
        .set_font_charset(178);

    let worksheet = workbook.add_worksheet();
    worksheet.write_string(0, 0, "Foo", &format1)?;

    workbook.close()?;

    Ok(())
}

#[test]
fn format13_font_charset() {
    let testcase = "format13";

    let (excel_file, xlsxwriter_file) = common::get_xlsx_filenames(testcase);
    _ = create_new_xlsx_file(&xlsxwriter_file);
    common::assert_eq(&excel_file, &xlsxwriter_file);
    common::remove_test_xlsx_file(&xlsxwriter_file);
}
