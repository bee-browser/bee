print(void undefined); ///=Value::UNDEFINED
print(void null); ///=Value::UNDEFINED
print(void true); ///=Value::UNDEFINED
print(void false); ///=Value::UNDEFINED
print(void 0); ///=Value::UNDEFINED
print(void NaN); ///=Value::UNDEFINED
print(void Infinity); ///=Value::UNDEFINED
print(void void 0); ///=Value::UNDEFINED

const a = 1;
print(void a); ///=Value::UNDEFINED
