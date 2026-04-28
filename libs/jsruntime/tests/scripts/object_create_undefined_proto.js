try {
  Object.create(undefined);
} catch (e) {
  print(e.name); ///="TypeError"
}
