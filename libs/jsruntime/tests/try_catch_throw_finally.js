let i = 0;
try {
  i += 1;
  throw i;
} catch (e) {
  i = e + 1;
  throw i;
} finally {
  // TODO: add code to check whether the finally clause is evaluated.
  i += 1;
}
