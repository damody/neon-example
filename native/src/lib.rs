#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js::{JsInteger, JsNumber, JsObject, JsString, JsArray, Object};

fn hello(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    Ok(JsString::new(scope, "hello node").unwrap())
}

fn adder(call: Call) -> JsResult<JsNumber> {
    let scope = call.scope;
    let a = call.arguments
        .require(scope, 0)?
        .check::<JsNumber>()?
        .value();
    let b = call.arguments
        .require(scope, 1)?
        .check::<JsNumber>()?
        .value();
    Ok(JsNumber::new(scope, a + b))
}

/// Takes in an object, returning a new object.
fn obj_adder(call: Call) -> JsResult<JsObject> {
    use neon::js::Object;

    let scope = call.scope;

    let obj = &call.arguments.require(scope, 0)?.check::<JsObject>()?;
    let a: f64 = obj.get(scope, "a")?.check::<JsNumber>()?.value();
    let b: f64 = obj.get(scope, "b")?.check::<JsNumber>()?.value();

    let result = JsObject::new(scope);
    result.set("result", JsNumber::new(scope, a + b))?;

    Ok(result)
}

fn make_an_array(call: Call) -> JsResult<JsArray> {
    let scope = call.scope; // the current scope for rooting handles
    let array = JsArray::new(scope, 3);
    try!(array.set(0, JsInteger::new(scope, 9000)));
    try!(array.set(1, JsObject::new(scope)));
    try!(array.set(2, JsNumber::new(scope, 3.14159)));
    Ok(array)
}

register_module!(m, {
    m.export("hello", hello)?;
    m.export("adder", adder)?;
    m.export("objAdder", obj_adder)?;
    m.export("make_an_array", make_an_array)?;
    Ok(())
});