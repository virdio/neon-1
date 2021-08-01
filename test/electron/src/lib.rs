use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("Hello, World!"))
}

// Crashes a few levels deep into Rust code, for testing that when this
// happens we get a usable stack backtrace.
fn crash(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    crash_f1();
    Ok(cx.undefined())
}
fn crash_f1() {
    crash_f2();
}
fn crash_f2() {
    assert_eq!(1,2);
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("crash_deep", crash)?;
    Ok(())
}
