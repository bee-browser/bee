print(typeof Object.defineProperties); ///="function"
print(Object.defineProperties.length); ///=2

const object = {};

Object.defineProperties(object, {
  property1: {
    value: 42,
    enumerable: true,
  },
  property2: {},
});

print(object.property1); ///=42
print(object.property2); ///=undefined
