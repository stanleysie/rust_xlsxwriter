// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{CustomSerializeField, SerializeFieldOptions, Workbook, XlsxError};
use serde::Serialize;

// Test case for Serde serialization. First test isn't serialized.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Not serialized.
    worksheet.write(0, 0, 123)?;
    worksheet.write(1, 0, 456)?;
    worksheet.write(2, 0, 789)?;
    worksheet.write(0, 1, true)?;
    worksheet.write(1, 1, false)?;
    worksheet.write(2, 1, true)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for Serde serialization.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Create a serializable test struct.
    #[derive(Serialize)]
    struct MyStruct {
        col1: Vec<u16>,
        col2: Vec<bool>,
    }

    let data = MyStruct {
        col1: vec![123, 456, 789],
        col2: vec![true, false, true],
    };

    let custom_headers = [
        CustomSerializeField::new("col1"),
        CustomSerializeField::new("col2"),
    ];
    let header_options = SerializeFieldOptions::new()
        .set_custom_headers(&custom_headers)
        .hide_headers(true);

    worksheet.serialize_headers_with_options(0, 0, &data, &header_options)?;

    worksheet.serialize(&data)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_serde09_1() {
    let test_runner = common::TestRunner::new()
        .set_name("serde09")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_serde09_2() {
    let test_runner = common::TestRunner::new()
        .set_name("serde09")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
