import {API,PAGE,ROUTER} from './base.js';

PAGE('mybooks', 'Meine Bücher', undefined, 9, PAGE._CONDITIONALS.onAuthenticated);

ROUTER
  .on('mybooks', ()=>PAGE._RENDER(()=>Promise.resolve({}),PAGE.mybooks));
