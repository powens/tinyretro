import '@testing-library/jest-dom/vitest';
import { expect } from 'vitest';
import { JSDOM } from 'jsdom';

// Create a new JSDOM instance
const dom = new JSDOM('', { url: 'http://localhost' });

// Set global variables to simulate a browser environment
global.window = dom.window;
global.document = dom.window.document;
global.navigator = dom.window.navigator;
global.HTMLElement = dom.window.HTMLElement;
global.Node = dom.window.Node;

// Mock DragEvent since it's not available in JSDOM
global.DragEvent = class DragEvent extends Event {
  constructor(type, eventInitDict) {
    super(type, eventInitDict);
    this.dataTransfer = eventInitDict?.dataTransfer || {
      setData: () => {},
      getData: () => '',
      effectAllowed: 'none',
    };
  }
};

// Mock matchMedia
global.window.matchMedia = global.window.matchMedia || function() {
  return {
    matches: false,
    addListener: function() {},
    removeListener: function() {},
  };
};

// Extend expect with additional matchers if needed
expect.extend({
  // Add custom matchers here
});