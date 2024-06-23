'use strict';

if ($OPTIONS.debug) {
  debugger;
}

if (!window.beeTools.domScraper.data) {
  if (!window.beeTools.domScraper.snapshot) {
    return undefined;
  }
  const json = JSON.stringify(window.beeTools.domScraper.snapshot);
  // Split the JSON into segments of 1M characters.
  window.beeTools.domScraper.data = json.match(/.{1,1000000}/g);
}

return window.beeTools.domScraper.data.shift();
