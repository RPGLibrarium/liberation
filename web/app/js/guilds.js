import {API,PAGE,ROUTER} from './base.js';

PAGE('guilds', 'Gilden', undefined, 3, PAGE._CONDITIONALS.onDev);

ROUTER
  .on('guilds', ()=>PAGE._RENDER(()=>Promise.resolve({}),PAGE.guilds));
