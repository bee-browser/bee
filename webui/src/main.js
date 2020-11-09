'use strict';

import DebugConsole from './debug_console';

const widget = new DebugConsole;
document.body.appendChild(widget.render());
widget.on('ready', () => widget.load('text:HELLO, WORLD.'));
widget.start();
