for (;;) {
  try {
    print(1); ///=1
    break;
  } catch {
    print(2);
  } finally {
    print(3); ///=3
  }
}
