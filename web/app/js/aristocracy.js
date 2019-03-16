import {API,PAGE,ROUTER} from './base.js';

PAGE('aristocracy', 'Aristokratie', 'peaks_of_aristocracy', 42, PAGE._CONDITIONALS.onAristocrat);

ROUTER
  .on('aristocracy', ()=>PAGE._RENDER(()=>Promise.resolve({}),PAGE.aristocracy));
