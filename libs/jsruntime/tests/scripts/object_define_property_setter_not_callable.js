try {
  Object.defineProperty({}, 'a', {
    set: null,
  });
} catch (e) {
  print(e.name); ///="TypeError"
}

try {
  Object.defineProperty({}, 'a', {
    set: {},
  });
} catch (e) {
  print(e.name); ///="TypeError"
}
