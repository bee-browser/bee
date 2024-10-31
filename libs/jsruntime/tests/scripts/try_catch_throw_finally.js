let i = 0;
print(i); ///=0
try {
  i += 1;
  print(i); ///=1
  throw i;
} catch (e) {
  i = e + 1;
  print(i); ///=2
  throw i; ///!2
} finally {
  i += 1;
  print(i); ///=3
}
