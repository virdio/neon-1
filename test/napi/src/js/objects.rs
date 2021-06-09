use neon::prelude::*;

pub fn return_js_global_object(mut cx: FunctionContext) -> JsResult<JsObject> {
    Ok(cx.global())
}

pub fn return_js_object(mut cx: FunctionContext) -> JsResult<JsObject> {
    Ok(cx.empty_object())
}

pub fn return_js_object_with_mixed_content(mut cx: FunctionContext) -> JsResult<JsObject> {
    let js_object: Handle<JsObject> = cx.empty_object();
    let n = cx.number(9000.0);
    js_object.set(&mut cx, "number", n)?;
    let s = cx.string("hello node");
    js_object.set(&mut cx, "string", s)?;
    Ok(js_object)
}

pub fn return_js_object_with_number(mut cx: FunctionContext) -> JsResult<JsObject> {
    let js_object: Handle<JsObject> = cx.empty_object();
    let n = cx.number(9000.0);
    js_object.set(&mut cx, "number", n)?;
    Ok(js_object)
}

pub fn return_js_object_with_string(mut cx: FunctionContext) -> JsResult<JsObject> {
    let js_object: Handle<JsObject> = cx.empty_object();
    let s = cx.string("hello node");
    js_object.set(&mut cx, "string", s)?;
    Ok(js_object)
}

pub fn return_array_buffer(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
    let b: Handle<JsArrayBuffer> = cx.array_buffer(16)?;
    Ok(b)
}

pub fn read_array_buffer_with_lock(_: FunctionContext) -> JsResult<JsNumber> {
    todo!()
}

pub fn read_array_buffer_with_borrow(_: FunctionContext) -> JsResult<JsNumber> {
    todo!()
}

pub fn sum_array_buffer_with_borrow(_: FunctionContext) -> JsResult<JsNumber> {
    todo!()
}

pub fn write_array_buffer_with_lock(_: FunctionContext) -> JsResult<JsUndefined> {
    todo!()
}

pub fn write_array_buffer_with_borrow_mut(_: FunctionContext) -> JsResult<JsUndefined> {
    todo!()
}

pub fn increment_array_buffer_with_borrow_mut(_: FunctionContext) -> JsResult<JsUndefined> {
    todo!()
}

pub fn return_uninitialized_buffer(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let b: Handle<JsBuffer> = unsafe { JsBuffer::uninitialized(&mut cx, 16)? };
    Ok(b)
}

pub fn return_buffer(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let b: Handle<JsBuffer> = cx.buffer(16)?;
    Ok(b)
}

pub fn return_external_buffer(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let data = cx.argument::<JsString>(0)?.value(&mut cx);
    let buf = JsBuffer::external(&mut cx, data.into_bytes());

    Ok(buf)
}

pub fn return_external_array_buffer(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
    let data = cx.argument::<JsString>(0)?.value(&mut cx);
    let buf = JsArrayBuffer::external(&mut cx, data.into_bytes());

    Ok(buf)
}

pub fn read_buffer_with_lock(_: FunctionContext) -> JsResult<JsNumber> {
    todo!()
}

pub fn read_buffer_with_borrow(_: FunctionContext) -> JsResult<JsNumber> {
    todo!()
}

pub fn sum_buffer_with_borrow(_: FunctionContext) -> JsResult<JsNumber> {
    todo!()
}

pub fn write_buffer_with_lock(_: FunctionContext) -> JsResult<JsUndefined> {
    todo!()
}

pub fn write_buffer_with_borrow_mut(_: FunctionContext) -> JsResult<JsUndefined> {
    todo!()
}

pub fn increment_buffer_with_borrow_mut(_: FunctionContext) -> JsResult<JsUndefined> {
    todo!()
}
