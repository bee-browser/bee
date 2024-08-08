print(x());

function x() {
  let a = 0;
  if (true) {
    a = 1;
    {
      throw a;
    }
    // never reach here
    a = 2;
  }
  // never reach here
  a = 3;
  return a;
}
