try {
  print(a);
} catch (e) {
  // TODO(feat): ReferenceError
  print(e); ///=1000
}

a = 1;
print(a); ///=1
