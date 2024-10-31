try {
  print(0); ///=0
  for (;;) {
    throw 2;
  }
  // never reach here
  print(1);
} catch (e) {
  print(e); ///=2
}
print(100); ///=100
