print(typeof Object.prototype.propertyIsEnumerable); ///="function"
print(Object.prototype.propertyIsEnumerable.length); ///=1

const o = { a: 1 };
print(o.propertyIsEnumerable('a')); ///=true
print(o.propertyIsEnumerable('b')); ///=false
print(Object.propertyIsEnumerable('prototype')); ///=false
