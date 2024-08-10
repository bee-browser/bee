try {
  let i = 0;
  {
    print(i); ///=0
    throw () => i;
  }
  // never reach here
  i = 1;
  print(i);
} catch (e) {
  print(e()); ///=0
  throw e() + 1; ///!1
}
// never reach here
print(100);
