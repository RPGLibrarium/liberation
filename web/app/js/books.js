import {API,PAGE,ROUTER} from './base.js';

PAGE('mybooks', 'Meine Bücher', undefined, 9);

ROUTER
  .on('mybooks', ()=>PAGE._RENDER(()=>Promise.resolve({}),PAGE.mybooks));
