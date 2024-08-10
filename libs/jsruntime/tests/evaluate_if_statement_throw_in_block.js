x();

function x() {
  print(0); ///=0
  if (true) {
    print(1); ///=1
    {
      throw 100; ///!100
    }
    // never reach here
    print(2);
  }
  // never reach here
  print(3);
}
