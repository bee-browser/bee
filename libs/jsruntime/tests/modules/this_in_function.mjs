function a() {
  // Always in the strict mode.
  return this;
}

print(a() === undefined); ///=true
print(a.call(null) === null); ///=true

const o = {};
print(a.call(o) === o); ///=true
