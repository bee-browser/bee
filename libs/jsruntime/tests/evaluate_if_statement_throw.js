x();

function x() {
  print(0); ///=0
  {
    print(1); ///=1
    if (true)
      throw 100; ///!100
    // never reach here
    print(2);
  }
  // never reach here
  print(3);
}
