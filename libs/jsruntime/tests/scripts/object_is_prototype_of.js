print(typeof Object.prototype.isPrototypeOf); ///="function"
print(Object.prototype.isPrototypeOf.length); ///=1

const o = {};
print(Object.prototype.isPrototypeOf(o)); ///=true
print(String.prototype.isPrototypeOf(o)); ///=false
