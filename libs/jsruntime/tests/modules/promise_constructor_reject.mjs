const p = new Promise((resolve, reject) => {
  reject(1);
});

try {
  await p;
} catch (e) {
  print(e); ///=1
}
