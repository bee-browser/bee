let i = 0;
try {
  throw 1; ///!1
} finally {
  i = 2;
  print(i); ///=2
}
// never reach here
print(i);
