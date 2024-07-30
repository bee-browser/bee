print(a());

function a() {
  let a = 1;
  {
    {
      return a;
    }
  }
}
