try {
  Object.defineProperty({}, 'a', {
    value: 1,
    get() {},
  });
} catch (e) {
  print(e.name); ///="TypeError"
}

try {
  Object.defineProperty({}, 'a', {
    value: 1,
    get: () => {},
  });
} catch (e) {
  print(e.name); ///="TypeError"
}

try {
  const o = Object.defineProperty({}, 'a', {
    value: 1,
    get: undefined,
  });
  print(o.a); ///=1
} catch (e) {
  print(e.name);
}
