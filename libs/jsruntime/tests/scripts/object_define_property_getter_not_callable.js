try {
  Object.defineProperty({}, 'a', {
    get: null,
  });
} catch (e) {
  print(e.name); ///="TypeError"
}

try {
  Object.defineProperty({}, 'a', {
    get: {},
  });
} catch (e) {
  print(e.name); ///="TypeError"
}
