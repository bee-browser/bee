try {
  let i = 0;
  {
    throw () => i;
  }
  i = 1;
} catch (e) {
  throw e() + 1; ///=1
}
