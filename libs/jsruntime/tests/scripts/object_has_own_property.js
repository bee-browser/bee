print(typeof Object.prototype.hasOwnProperty); ///="function"
print(Object.prototype.hasOwnProperty.length); ///=1

const o = { a: 1 };
print(o.hasOwnProperty('a')); ///=true
print(o.hasOwnProperty('hasOwnProperty')); ///=false
