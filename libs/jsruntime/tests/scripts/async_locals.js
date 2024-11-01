x();

async function x() {
  let a = 1;
  print(a); ///=1
  await 0;
  print(a + a); ///=2
}
