import {API,PAGE,ROUTER,keycloak} from './base.js';

PAGE('profile', 'Mein Profil', 'profile_details', -2, PAGE._CONDITIONALS.onAuthenticated);

PAGE('login', 'Login', undefined, -1, PAGE._CONDITIONALS.onNotAuthenticated);
PAGE('logout', 'Logout', undefined, -1, PAGE._CONDITIONALS.onAuthenticated);

ROUTER
  .on('profile', ()=>PAGE._RENDER(loadProfileData, PAGE.profile));

function loadProfileData() {
  if(!(keycloak && keycloak.authenticated)) return Promise.resolve({
    authenticated: false,
  });
  const token = keycloak.tokenParsed;
  return Promise.resolve({
    authenticated: true,
    uid: token.uid,
    name: token.name,
    email: token.email,
    user: token.preferred_username,
    roles: token.roles,
  });
}
