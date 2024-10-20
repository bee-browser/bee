print(a()); ///=1

function a() {
  return b();

  function b() {
    return 1;
  }
}
