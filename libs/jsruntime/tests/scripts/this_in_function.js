function a() {
  return this;
}

print(a() === globalThis); ///=true

// TODO(feat): strict mode
// function b() {
//   "use strict";
//   return this;
// }

// print(b() === undefined); ///=true
