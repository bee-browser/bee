test(await undefined, await null, await true, await 1, await {}, await x, y(), await 300);

async function test(undef, nul, bool, number, object, closure, promise, last) {
  print(undef);         ///=undefined
  print(nul);           ///=null
  print(bool);          ///=true
  print(number);        ///=1
  print(object);        ///=object
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
