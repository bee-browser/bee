'use strict';

import * as csv from 'std/encoding/csv.ts';
import * as io from 'std/io/mod.ts';
import * as zip from 'zipjs';

zip.configure({
  // Deno supports only "module" type workers.
  useWebWorkers: false,
});

const DOWNLOADERS = {
  alexa: downloadAlexaTop1M,
  majestic: downloadMajesticMillion,
};

export const RANKINGS = Object.keys(DOWNLOADERS).sort();

export async function show(ranking, limit) {
  let urls = await DOWNLOADERS[ranking]();
  urls.slice(0, limit).forEach((url) => console.log(url));
}

async function downloadAlexaTop1M() {
  const url = 'http://s3.amazonaws.com/alexa-static/top-1m.csv.zip';
  const reader = new zip.ZipReader(new zip.HttpReader(url));
  const entries = await reader.getEntries();
  const csv = await entries[0].getData(new zip.TextWriter());
  return csv.trim().split('\n').map((line) => line.split(',')[1]);
}

async function downloadMajesticMillion() {
  const url = 'https://downloads.majestic.com/majestic_million.csv';
  const res = await fetch(url);
  const reader = new io.BufReader(io.readerFromStreamReader(res.body.getReader()));
  // TODO: very slow
  return await csv.parse(reader, {
    skipFirstRow: true,
    parse: (data) => data.Domain,
  });
}