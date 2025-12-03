const p = new Promise((resolve, reject) => {
  throw 1;
});

try {
  await p;
} catch (e) {
  print(e); ///=1
}
