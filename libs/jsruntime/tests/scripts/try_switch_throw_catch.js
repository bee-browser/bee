try {
  print(0); ///=0
  switch (0) {
    case 0:
      throw 2;
  }
  // never reach here
  print(1);
} catch (e) {
  print(e); ///=2
}
print(100); ///=100
