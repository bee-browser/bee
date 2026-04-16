async function a() {
  throw 1;
}

async function b() {
  await a();
}

try {
  await b();
} catch (e) {
  print(e); ///=1
}
