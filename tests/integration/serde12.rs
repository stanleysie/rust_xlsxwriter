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
    worksheet.set_paper_size(9);
    worksheet.set_column_width(1, 13.57)?;

    // Not serialized.
    worksheet.write(0, 0, "col1")?;
    worksheet.write(1, 0, 1)?;
    worksheet.write(2, 0, 2)?;
    worksheet.write(3, 0, 3)?;

    worksheet.write(0, 1, "col2")?;
    worksheet.write(1, 1, 4)?;
    worksheet.write(2, 1, 5)?;
    worksheet.write(3, 1, 6)?;

    worksheet.write(0, 2, "col3")?;
    worksheet.write(1, 2, 7)?;
    worksheet.write(2, 2, 8)?;
    worksheet.write(3, 2, 9)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for Serde serialization. Set the column width.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_paper_size(9);

    // Create a serializable test struct.
    #[derive(Serialize)]
    struct MyStruct {
        col1: u8,
        col2: u8,
        col3: u8,
    }

    let data1 = MyStruct {
        col1: 1,
        col2: 4,
        col3: 7,
    };

    let data2 = MyStruct {
        col1: 2,
        col2: 5,
        col3: 8,
    };

    let data3 = MyStruct {
        col1: 3,
        col2: 6,
        col3: 9,
    };

    let header_options = SerializeFieldOptions::new()
        .set_custom_headers(&[CustomSerializeField::new("col2").set_column_width(13.57)]);

    worksheet.serialize_headers_with_options(0, 0, &data1, &header_options)?;

    worksheet.serialize(&data1)?;
    worksheet.serialize(&data2)?;
    worksheet.serialize(&data3)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for Serde serialization. Set the column width.
fn create_new_xlsx_file_3(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_paper_size(9);

    // Create a serializable test struct.
    #[derive(Serialize)]
    struct MyStruct {
        col1: u8,
        col2: u8,
        col3: u8,
    }

    let data1 = MyStruct {
        col1: 1,
        col2: 4,
        col3: 7,
    };

    let data2 = MyStruct {
        col1: 2,
        col2: 5,
        col3: 8,
    };

    let data3 = MyStruct {
        col1: 3,
        col2: 6,
        col3: 9,
    };

    let header_options = SerializeFieldOptions::new()
        .set_custom_headers(&[CustomSerializeField::new("col2").set_column_width_pixels(100)]);

    worksheet.serialize_headers_with_options(0, 0, &data1, &header_options)?;

    worksheet.serialize(&data1)?;
    worksheet.serialize(&data2)?;
    worksheet.serialize(&data3)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_serde12_1() {
    let test_runner = common::TestRunner::new()
        .set_name("serde12")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_serde12_2() {
    let test_runner = common::TestRunner::new()
        .set_name("serde12")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_serde12_3() {
    let test_runner = common::TestRunner::new()
        .set_name("serde12")
        .set_function(create_new_xlsx_file_3)
        .unique("3")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
