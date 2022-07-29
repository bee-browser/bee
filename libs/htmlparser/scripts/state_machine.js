import { DOMParser } from 'https://deno.land/x/deno_dom/deno-dom-wasm.ts';

const url = 'https://html.spec.whatwg.org/multipage/parsing.html';

try {
  const res = await fetch(url);
  const html = await res.text();
  const document = new DOMParser().parseFromString(html, 'text/html');
  // Remove TOC
  document.querySelector('.toc').remove();
  for (let node of document.querySelectorAll('.secno')) {
    const secno = node.textContent;
    if (secno.startsWith('13.2.5.')) {
      console.error(`${secno}...`);
      const id = node.parentElement.id;
      const name = node.nextElementSibling.textContent;
      const nodes = node.parentElement.nextElementSibling.nextElementSibling.children;
      const table = [];
      let charClass = [];
      for (let i = 0; i < nodes.length; ++i) {
        if (nodes[i].tagName === 'DT') {
          charClass.push(nodes[i].textContent);
        } else {
          const action = nodes[i].textContent;
          table.push({ charClass, action });
          charClass = [];
        }
      }
      console.log(JSON.stringify({
        secno,
        id,
        name,
        table,
      }));
    }
  }
} catch(error) {
  console.error(error);
}
