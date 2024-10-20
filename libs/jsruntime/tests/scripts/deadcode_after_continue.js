let i = 0;
for (; i < 2; ++i) {
  continue;
  // never reach here
  print(0);
}
print(i); ///=2
