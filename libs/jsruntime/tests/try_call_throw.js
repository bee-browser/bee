let i = 0;
try {
  a(i);
} catch (e) {
  i = e;
}
print(i);

function a(i) {
  throw i + 1;
}
