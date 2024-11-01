x(1);

async function x(a) {
  print(a); ///=1
  await 0;
  print(a + a); ///=2
}
