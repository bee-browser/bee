function a() {
  // Always in the strict mode.
  return this;
}

print(a() === undefined); ///=true
