print(a()); ///=1

function a() {
  let a = 1;
  {
    {
      return a;
    }
  }
}
