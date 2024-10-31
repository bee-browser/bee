try {
  print(0); ///=0
  if (true)
    throw 5; ///!5
  // never reach here
  print(1);
} finally {
  print(2); ///=2
}
// never reach here
print(100);
