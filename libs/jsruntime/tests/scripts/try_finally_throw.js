let i = 0;
try {
  throw 1;
} finally {
  throw 2; ///!2
}
