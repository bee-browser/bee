let a = {
  await: await 0,
  async: async function() { return 1; },
  asyncArrow: async () => 2,
};

print(a.await); ///=0
print(typeof a.async); ///="function"
print(await a.async()); ///=1
print(typeof a.asyncArrow); ///="function"
print(await a.asyncArrow()); ///=2
