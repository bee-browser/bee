'use strict';

if ($OPTIONS.debug) {
  debugger;
}

class Scraper {
  // public

  constructor(document) {
    this.document_ = document;
    this.assets_ = {};
    this.assetUrlMap_ = {};
  }

  async scrape() {
    const document = {
      url: this.document_.URL,
      title: this.document_.title,
      root: await this.scrapeNode_(this.document_.documentElement),
    };

    return {
      document,
      assets: this.assets_,
      viewport: {
        width: window.innerWidth,
        height: window.innerHeight,
      },
    };
  }

  // private

  async scrapeNode_(node) {
    switch (node.nodeType) {
    case Node.ELEMENT_NODE:
      return await this.scrapeElement_(node);
    case Node.TEXT_NODE:
      return await this.scrapeTextNode_(node);
    default:
      // TODO: throw an exception?
      return null;
    }
  }

  async scrapeElement_(element) {
    const id = Scraper.getNextNodeId_();
    const style = this.scrapeComputedStyle_(element);

    await this.collectAssets_(element, style);

    let childNodes = [];
    childNodes.push(await this.scrapePseudoElement_(element, '::before'));
    for (let i = 0; i < element.childNodes.length; ++i) {
      childNodes.push(await this.scrapeNode_(element.childNodes[i]));
    }
    childNodes.push(await this.scrapePseudoElement_(element, '::after'));
    childNodes = childNodes.filter((node) => node !== null);  // remove nulls

    // Should we remove an empty 'style' attribute which has created in
    // `Scraper.scrapeComputedStyle_()`?  See comments in this method for details.
    let attributes = {};
    for (let i = 0; i < element.attributes.length; ++i) {
      const attr = element.attributes.item(i);
      attributes[attr.name] = attr.value;
    }

    return {
      type: 'dom.element',
      id,
      tagName: element.tagName,
      attributes,
      style,
      childNodes,
    };
  }

  async scrapePseudoElement_(element, pseudo) {
    const style = this.scrapeComputedStyle_(element, pseudo);
    if (style === null) {
      return null;
    }

    await this.collectAssets_(null, style);

    return {
      type: 'dom.pseudo_element',
      id: Scraper.getNextNodeId_(),
      pseudo,
      style,
      childNodes: [],
    };
  }

  // We use window.getComputedStyle() because we cannot use Element.computedStyleMap()
  // for getting styles of a pseudo-element.
  scrapeComputedStyle_(element, pseudo = null) {
    const style = window.getComputedStyle(element, pseudo);
    if (pseudo !== null && style.content === 'none') {
      return null;
    }

    const display = style.display;
    const saved = element.style.display;
    // Change the value of the display property to 'none' temporarily, in order to get
    // "computed" values.  See https://stackoverflow.com/questions/9730612.
    //
    // NOTE
    // ----
    // The following procedures make a harmless side effect that the empty 'style' attribute will
    // be created on the element if it doesn't exist.  However, it doesn't change the rendering
    // result.
    element.style.display = 'none';
    let result = this.scrapeStyleProperties_(style);
    element.style.display = saved;
    result.display = display;

    // TODO: convert attributes into internal properties

    return result;
  }

  // Simply copy properties.
  scrapeStyleProperties_(style) {
    let result = {};

    for (let i = 0; i < style.length; ++i) {
      const prop = style[i];
      result[prop] = style.getPropertyValue(prop);
    }

    return result;
  }

  async scrapeTextNode_(textNode) {
    return {
      type: 'dom.text',
      id: Scraper.getNextNodeId_(),
      text: textNode.nodeValue,
    };
  }

  async collectAssets_(element, style) {
    switch (element?.tagName) {
    case 'IMG':
      {
        let url = element.currentSrc;
        if (url === '') {
          url = Scraper.convertImageToDataUrl_(element);
        }
        let id = this.assetUrlMap_[url];
        if (id === undefined) {
          id = Scraper.getNextAssetId_();
          this.assets_[id] = {
            id,
            url,
            type: 'image',
            width: element.naturalWidth,
            height: element.naturalHeight,
          };
          this.assetUrlMap_[url] = id;
        }
        style['-bee-content-asset-id'] = id;
      }
      break;
    case 'VIDEO':
      {
        const url = element.currentSrc;
        let id = this.assetUrlMap_[url];
        if (id === undefined) {
          id = Scraper.getNextAssetId_();
          this.assets_[id] = {
            id,
            url,
            type: 'video',
            width: element.videoWidth,
            height: element.videoHeight,
          };
          this.assetUrlMap_[url] = id;
        }
        style['-bee-content-asset-id'] = id;
      }
      break;
    case 'CANVAS':
      {
        const url = element.toDataURL('image/png');
        let id = this.assetUrlMap_[url];
        if (id === undefined) {
          id = Scraper.getNextAssetId_();
          this.assets_[id] = {
            id,
            url,
            type: 'canvas',
            width: element.width,
            height: element.height,
          };
          this.assetUrlMap_[url] = id;
        }
        style['-bee-content-asset-id'] = id;
      }
      break;
    case 'OBJECT':
      // TODO
      break;
    }
  }

  static nextNodeId_ = 1;  // unique in the same document

  static getNextNodeId_() {
    const id = this.nextNodeId_++;
    return id;
  }

  static nextAssetId_ = 1;  // unique in the same document

  static getNextAssetId_() {
    const id = this.nextAssetId_++;
    return id;
  }

  static convertImageToDataUrl_(image) {
    try {
      const canvas = document.createElement('canvas');
      canvas.width = image.naturalWidth;
      canvas.height = image.naturalHeight;
      canvas.getContext('2d').drawImage(image, 0, 0);
      return canvas.toDataURL('image/png');
    } catch (err) {
      const canvas = document.createElement('canvas');
      canvas.width = image.naturalWidth;
      canvas.height = image.naturalHeight;
      const context = canvas.getContext('2d');
      context.fillStyle = Scraper.randomColor_();
      context.fillRect(0, 0, canvas.witdh, canvas.height);
      return canvas.toDataURL('image/png');
    }
  }

  static randomColor_() {
    const n = Math.random() * 0xFFFFFF + 0x1000000;
    return `#${n.toString(16).substr(1, 6)}`;
  }
}

async function takeSnapshot() {
  // TODO: frames
  const scraper = new Scraper(document);
  window.beeTools = {
    domScraper: {
      snapshot: await scraper.scrape(),
    },
  };
}

return new Promise(async (resolve, reject) => {
  try {
    resolve(await takeSnapshot());
  } catch (e) {
    reject(e);
  }
});
