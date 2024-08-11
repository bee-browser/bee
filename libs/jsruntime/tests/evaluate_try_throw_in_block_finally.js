try {
  let i = 0;
  {
    throw 1; ///!1
  }
  i = 1;
} finally {
  let i = 0;
}
