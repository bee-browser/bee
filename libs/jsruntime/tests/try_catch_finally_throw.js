let i = 0;
try {
  i += 1;
  throw i;
} catch (e) {
  i = e + 1;
} finally {
  i += 1;
  throw i;
}
