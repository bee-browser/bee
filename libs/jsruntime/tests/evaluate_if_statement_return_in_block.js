x();

function x() {
  print(0); ///=0
  if (true) {
    print(1); ///=1
    {
      print(2); ///=2
      return;
    }
    // never reach here
    print(3);
  }
  // never reach here
  print(4);
}
