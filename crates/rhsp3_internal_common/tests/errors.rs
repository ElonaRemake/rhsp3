use rhsp3_internal_common::errors::*;

fn check_ultimate_source(error: Result<()>, is_std: bool, count: u32) {
    let error = error.expect_err("Result is not an error.");

    let mut source_opt = dbg!(error.source());
    let mut ultimate_source = None;
    let mut source_count = 0;
    while let Some(source) = source_opt {
        ultimate_source = source_opt.take();
        source_opt = dbg!(source.source());
        source_count += 1;
    }
    println!();

    assert_eq!(source_count, count);
    if count != 0 {
        if is_std {
            let err = ultimate_source.unwrap().downcast_ref::<std::io::Error>().unwrap();
            assert_eq!(err.kind(), std::io::ErrorKind::Other);
            assert!(err.to_string().contains("Testing error"));
        } else {
            let err = ultimate_source.unwrap().downcast_ref::<ErrorWrapper>().unwrap();
            assert_eq!(err.to_string(), "[Testing error]");
        }
    }
}

fn check_no_backtrace(error: Result<()>) {
    assert!(error
        .expect_err("Result is not an error.")
        .backtrace()
        .is_none());
}
fn check_backtrace_contains(error: Result<()>, target_func: &str) {
    let error = error.expect_err("Result is not an error.");
    let mut backtrace = error
        .backtrace()
        .expect("Error does not contain a backtrace!")
        .clone();
    backtrace.resolve();

    println!("=== Expecting: {} ===", target_func);
    println!("Error (Display): {}", error);
    println!("Error (Debug): {:?}", error);
    println!();
    println!("{:?}", backtrace);
    println!();

    assert!(format!("{:?}", backtrace).contains(target_func));
}

#[inline(never)]
fn bt_root_kind() -> Result<()> {
    #[inline(never)]
    fn inner() -> Result<()> {
        Err(error_new(ErrorKind::TestingError).with_backtrace())
    }

    let x = inner();
    println!("-"); // used to prevent TCO
    x
}
#[inline(never)]
fn bt_root_std() -> Result<()> {
    #[inline(never)]
    fn inner() -> Result<()> {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Testing error"))?;
        Ok(()) // unreachable
    }

    let x = inner();
    println!("-"); // used to prevent TCO
    x
}
#[inline(never)]
fn bt_root_no_backtrace() -> Result<()> {
    Err(error_new(ErrorKind::TestingError))
}

#[test]
fn error_ultimate_source_is_correct() {
    // Check with wrapped std errors
    check_ultimate_source(bt_root_std(), true, 1);
    check_ultimate_source(bt_root_std().context(ErrorKind::TestingError), true, 1);
    check_ultimate_source(
        bt_root_std()
            .context(ErrorKind::TestingError)
            .context(ErrorKind::TestingError),
        true,
        2,
    );

    // Check with errors from rhsp3
    check_ultimate_source(bt_root_kind(), false, 0);
    check_ultimate_source(bt_root_kind().context(ErrorKind::TestingError), false, 1);
    check_ultimate_source(
        bt_root_kind()
            .context(ErrorKind::TestingError)
            .context(ErrorKind::TestingError),
        false,
        2,
    );
    check_ultimate_source(
        bt_root_kind()
            .context(ErrorKind::TestingError)
            .context(ErrorKind::TestingError)
            .context(ErrorKind::TestingError),
        false,
        3,
    );
}

#[test]
fn no_unexpected_backtraces() {
    check_no_backtrace(bt_root_no_backtrace());
    check_no_backtrace(bt_root_no_backtrace().context(ErrorKind::TestingError));
    check_no_backtrace(
        bt_root_no_backtrace()
            .context(ErrorKind::TestingError)
            .context(ErrorKind::TestingError),
    );
}

#[test]
fn error_backtrace_contains_bt_root() {
    // Check with wrapped std errors
    check_backtrace_contains(bt_root_std(), "bt_root_std");
    check_backtrace_contains(bt_root_std().context(ErrorKind::TestingError), "bt_root_std");
    check_backtrace_contains(
        bt_root_std()
            .context(ErrorKind::TestingError)
            .context(ErrorKind::TestingError),
        "bt_root_std",
    );

    // Check with errors from rhsp3
    check_backtrace_contains(bt_root_kind(), "bt_root_kind");
    check_backtrace_contains(bt_root_kind().context(ErrorKind::TestingError), "bt_root_kind");
    check_backtrace_contains(
        bt_root_kind()
            .context(ErrorKind::TestingError)
            .context(ErrorKind::TestingError),
        "bt_root_kind",
    );
}
