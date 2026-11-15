function a() {
  return this;
}

print(a() === globalThis); ///=true
print(a.call(null) === globalThis); ///=true

const o = {};
print(a.call(o) === o); ///=true

// TODO(feat): strict mode
// function b() {
//   "use strict";
//   return this;
// }

// print(b() === undefined); ///=true
