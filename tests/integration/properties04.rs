// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

#[cfg(feature = "chrono")]
use chrono::{TimeZone, Utc};

use crate::common;
use rust_xlsxwriter::{DocProperties, ExcelDateTime, Workbook, XlsxError};

// Test to demonstrate document properties.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let date = ExcelDateTime::from_ymd(2016, 12, 12)?.and_hms(23, 0, 0)?;

    let properties = DocProperties::new()
        .set_custom_property("Checked by".to_string(), "Adam".to_string())
        .set_custom_property("Date completed", &date)
        .set_custom_property("Document number", 12345)
        .set_custom_property("Reference", 1.2345)
        .set_custom_property("Source".to_string(), true)
        .set_custom_property("Status", false)
        .set_custom_property("Department", "Finance")
        .set_custom_property("Group", 1.2345678901234);

    workbook.set_properties(&properties);

    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(0, 70)?;
    worksheet.write_string(
        0,
        0,
        r#"Select 'Office Button -> Prepare -> Properties' to see the file properties."#,
    )?;
    workbook.save(filename)?;
    workbook.save(filename)?;

    Ok(())
}

// Test to demonstrate document properties. With Chrono.
#[cfg(feature = "chrono")]
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let date = Utc.with_ymd_and_hms(2016, 12, 12, 23, 0, 0).unwrap();

    let properties = DocProperties::new()
        .set_custom_property("Checked by".to_string(), "Adam".to_string())
        .set_custom_property("Date completed", &date)
        .set_custom_property("Document number", 12345)
        .set_custom_property("Reference", 1.2345)
        .set_custom_property("Source".to_string(), true)
        .set_custom_property("Status", false)
        .set_custom_property("Department", "Finance")
        .set_custom_property("Group", 1.2345678901234);

    workbook.set_properties(&properties);

    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(0, 70)?;
    worksheet.write_string(
        0,
        0,
        r#"Select 'Office Button -> Prepare -> Properties' to see the file properties."#,
    )?;
    workbook.save(filename)?;
    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_properties04_1() {
    let test_runner = common::TestRunner::new()
        .set_name("properties04")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[cfg(feature = "chrono")]
#[test]
fn test_properties04_2() {
    let test_runner = common::TestRunner::new()
        .set_name("properties04")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
