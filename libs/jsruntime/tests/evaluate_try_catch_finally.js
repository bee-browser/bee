let i = 0;
try {
  throw 1;
} catch (e) {
  i = e + 1;
} finally {
  i = 3;
}
print(i); ///=3
