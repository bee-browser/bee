print(x());

function x() {
  let a = 0;
  {
    a = 1;
    if (true)
      throw a;
    // never reach here
    a = 1;
  }
  // never reach here
  a = 2;
  return a;
}
