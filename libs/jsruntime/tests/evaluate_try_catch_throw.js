let i = 0;
try {
  throw 1;
} catch (e) {
  throw e + 1; ///!2
}
// never reach here
print(100);
