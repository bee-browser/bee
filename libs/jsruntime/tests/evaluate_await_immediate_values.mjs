print(await undefined); ///=Value::UNDEFINED
print(await null); ///=Value::NULL
print(await true); ///=true
print(await false); ///=false
print(await 0); ///=0
print(await NaN); ///=f64::NAN
print(await Infinity); ///=f64::INFINITY

let a = await 1;
print(a); ///=1
