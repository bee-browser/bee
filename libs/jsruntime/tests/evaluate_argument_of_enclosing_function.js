print(a(1)); ///=1

function a(x) {
  return b();

  function b() {
    return x;
  }
}
