#![feature(c_unwind)]

use rhsp3_common::Result;
use rhsp3_plugsdk::{hsp_export, VarBuffer};

#[hsp_export]
fn test_function1() -> Result<()> {
    Ok(())
}

#[hsp_export]
fn test_function2(_: &mut impl VarBuffer) -> Result<()> {
    Ok(())
}

rhsp3_plugsdk::hpi_root!(UnifyFunctionsTest, [exrand_hpi::ExrandHpi]);

#[test]
fn test_codegen() {
    let header = rhsp3_plugsdk::codegen::make_hpi_headers::<UnifyFunctionsTest>("test.hpi");
    assert!(header.contains("test_function1"));
    assert!(header.contains("test_function2"));
    assert!(header.contains("ex_randomize_time"));
}
