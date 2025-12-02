const p = new Promise((resolve, reject) => {
  resolve(1);
});

print(await p); ///=1
