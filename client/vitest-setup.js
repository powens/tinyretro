import '@testing-library/jest-dom/vitest';

// Mock DragEvent if not provided by happy-dom
if (typeof globalThis.DragEvent === 'undefined') {
  globalThis.DragEvent = class DragEvent extends Event {
    constructor(type, eventInitDict) {
      super(type, eventInitDict);
      this.dataTransfer = eventInitDict?.dataTransfer || {
        setData: () => {},
        getData: () => '',
        effectAllowed: 'none',
      };
    }
  };
}

// Mock matchMedia if not provided by happy-dom
if (typeof globalThis.window?.matchMedia !== 'function') {
  globalThis.window.matchMedia = function() {
    return {
      matches: false,
      addListener: function() {},
      removeListener: function() {},
    };
  };
}