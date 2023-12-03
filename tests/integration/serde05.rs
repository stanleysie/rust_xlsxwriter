// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Workbook, XlsxError};
use serde::Serialize;

// Test case for Serde serialization. First test isn't serialized.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Not serialized.
    worksheet.write(0, 0, "col1")?;
    worksheet.write(1, 0, 123)?;
    worksheet.write(2, 0, 456)?;
    worksheet.write(3, 0, 789)?;
    worksheet.write(0, 1, "col2")?;
    worksheet.write(1, 1, true)?;
    worksheet.write(2, 1, false)?;
    worksheet.write(3, 1, true)?;

    worksheet.write(0, 3, "col1")?;
    worksheet.write(1, 3, 1)?;
    worksheet.write(2, 3, 2)?;
    worksheet.write(0, 4, "col2")?;
    worksheet.write(1, 4, 6)?;
    worksheet.write(2, 4, 7)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for Serde serialization. The test structs have similar field names
// to test for overwriting.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Create a serializable test struct.
    #[derive(Serialize)]
    struct MyStruct1 {
        col1: Vec<u16>,
        col2: Vec<bool>,
    }

    let data1 = MyStruct1 {
        col1: vec![123, 456, 789],
        col2: vec![true, false, true],
    };

    worksheet.write_serialize_headers(0, 0, &data1)?;
    worksheet.serialize(&data1)?;

    #[derive(Serialize)]
    struct MyStruct2 {
        col1: Vec<u8>,
        col2: Vec<u8>,
    }

    let data2 = MyStruct2 {
        col1: vec![1, 2],
        col2: vec![6, 7],
    };

    worksheet.write_serialize_headers(0, 3, &data2)?;
    worksheet.serialize(&data2)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for Serde serialization. The test structs have similar field names
// to test for overwriting. Additional the serialize calls are interleaved.
fn create_new_xlsx_file_3(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Create a serializable test struct.
    #[derive(Serialize)]
    struct MyStruct1 {
        col1: Vec<u16>,
        col2: Vec<bool>,
    }

    #[derive(Serialize)]
    struct MyStruct2 {
        col1: Vec<u8>,
        col2: Vec<u8>,
    }

    let data1 = MyStruct1 {
        col1: vec![123, 456, 789],
        col2: vec![true, false, true],
    };

    let data2 = MyStruct2 {
        col1: vec![1, 2],
        col2: vec![6, 7],
    };

    worksheet.write_serialize_headers(0, 0, &data1)?;
    worksheet.write_serialize_headers(0, 3, &data2)?;

    worksheet.serialize(&data1)?;
    worksheet.serialize(&data2)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_serde05_1() {
    let test_runner = common::TestRunner::new()
        .set_name("serde05")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_serde05_2() {
    let test_runner = common::TestRunner::new()
        .set_name("serde05")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_serde05_3() {
    let test_runner = common::TestRunner::new()
        .set_name("serde05")
        .set_function(create_new_xlsx_file_3)
        .unique("3")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
