// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{DocProperties, Workbook, XlsxError};

// Test case to demonstrate setting document properties.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let properties = DocProperties::new().set_author("Juan García Madero");

    workbook.set_properties(&properties);

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap56_doc_properties() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap56")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
