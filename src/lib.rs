use neon::prelude::*;


fn multiply(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let x = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let y = cx.argument::<JsNumber>(1)?.value(&mut cx);
    Ok(cx.number(x * y))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("multiply", multiply)?;
    Ok(())
}
