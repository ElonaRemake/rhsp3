#![feature(c_unwind)]

use rhsp3_common::Result;
use rhsp3_plugsdk::hsp_export;

#[hsp_export]
fn test_function() -> Result<()> {
    Ok(())
}

rhsp3_plugsdk::hpi_root!(UnifyFunctionsTest, [exrand_hpi::ExrandHpi]);

#[test]
fn test_codegen() {
    let header = rhsp3_plugsdk::codegen::make_hpi_headers::<UnifyFunctionsTest>("test.hpi");
    assert!(header.contains("test_function"));
    assert!(header.contains("ex_randomize_time"));
}
