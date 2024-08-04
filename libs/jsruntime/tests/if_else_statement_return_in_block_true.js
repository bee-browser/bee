print(x(true));

function x(cond) {
  let a = 0;
  if (cond) {
    a = 1;
    {
      return a;
    }
    // never reach here
    a = 2;
  } else {
    a = 3;
    {
      return a;
    }
    // never reach here
    a = 4;
  }
  // never reach here
  a = 5;
  return a;
}
