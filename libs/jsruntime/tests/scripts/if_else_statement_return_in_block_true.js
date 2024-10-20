a(true);

function a(cond) {
  print(0); ///=0
  if (cond) {
    print(1); ///=1
    {
      return;
    }
    // never reach here
    print(2);
  } else {
    print(3);
    {
      return;
    }
    // never reach here
    print(4);
  }
  // never reach here
  print(5);
}
