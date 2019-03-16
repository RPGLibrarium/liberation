import {API,PAGE,ROUTER} from './base.js';

PAGE('guilds', 'Gilden', undefined, 3);

ROUTER
  .on('guilds', ()=>PAGE._RENDER(()=>Promise.resolve({}),PAGE.guilds));
