let i = 0;
try {
  a(i);
} catch (e) {
  i = e;
}
print(i); ///=1

function a(i) {
  throw i + 1;
}
