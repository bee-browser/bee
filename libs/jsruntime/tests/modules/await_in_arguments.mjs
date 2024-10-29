test(await undefined, await null, await true, await 1, await x, y(), await 300);

async function test(undef, nul, bool, number, closure, promise, last) {
  print(undef);         ///=Value::UNDEFINED
  print(nul);           ///=Value::NULL
  print(bool);          ///=true
  print(number);        ///=1
  closure();            ///=100
  print(await promise); ///=200
  print(last);          ///=300
}

function x() {
  print(100);
}

async function y() {
  await 0;
  return 200;
}
