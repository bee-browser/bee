try {
  Object.assign(null);
} catch (e) {
  print(e.name); ///="TypeError"
}
