import {API,PAGE,ROUTER} from './base.js';

PAGE('profile', 'Mein Profil', undefined, -2, PAGE._CONDITIONALS.onAuthenticated);

PAGE('login', 'Login', undefined, -1, PAGE._CONDITIONALS.onNotAuthenticated);
PAGE('logout', 'Logout', undefined, -1, PAGE._CONDITIONALS.onAuthenticated);

ROUTER
  .on('profile', ()=>PAGE._RENDER(()=>Promise.resolve({}),PAGE.profile));
