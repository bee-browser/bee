let a = {
  await: await 0,
  async: async function() { return 1; },
  asyncArrow: async () => 2,
};

print(a.await); ///=0
print(a.async); ///=closure
print(await a.async()); ///=1
print(a.asyncArrow); ///=closure
print(await a.asyncArrow()); ///=2
