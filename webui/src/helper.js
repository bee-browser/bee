'use strict';

export function h(tagName, attrs = {}, ...children) {
  const elem = document.createElement(tagName);
  for (const attr of Object.entries(attrs)) {
    elem.setAttribute(attr[0], attr[1]);
  }
  for (const child of children) {
    elem.appendChild(child);
  }
  return elem;
}

export function t(text) {
  return document.createTextNode(text);
}

export function f(...children) {
  const frag = document.createDocumentFragment();
  for (const child of children) {
    frag.appendChild(child);
  }
  return frag;
}

export function formatBeeNumber(number) {
  return (number / 1024).toString();
}

export function formatBeePoint(point) {
  return `(${formatBeeNumber(point.x)}, ${formatBeeNumber(point.y)})`;
}

export function formatBeeRect(rect) {
  return `(${rect.x}, ${rect.y}) ${rect.width}x${rect.height}`;
}
