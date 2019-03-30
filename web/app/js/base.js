const WHOOSH_DURATION = 1000;
const execAfter = setTimeout;

// #######
// PAGES #
// #######
export const TEMPLATES = {};
let PAGES = {};
const ALL_PAGES = [];
export const PAGE = (page, title, template, nav=undefined, conditional=undefined)=>{
  if(PAGES[page]) return;
  conditional = conditional || (()=>true);

  let obj = {page,title,template,conditional};
  switch (typeof nav){
    case 'number': // position in navigation bar -> this is a MASTER PAGE!
      if(!Number.isSafeInteger(nav)) console.warn(`oh no! nav looks like a number, but is evil!`, nav);
      else obj.navPos = nav;
      break;
    case 'string': obj.navActive = nav; break; // associated MASTER PAGE to highlight in nav bar for the current sub page
    case 'undefined': break; // TODO remove maybe ...
    default: console.warn(`welp, we've got problems. nav has bad type '${typeof nav}' ... `, nav);
  }
  console.debug(obj);
  PAGES[page] = obj;
  ALL_PAGES.push(obj); // pushing hard
};
PAGES = PAGE;
let NAV_ACTIVE = 'librarium'; //Sane default value, is overwritten later on
PAGES._CONDITIONALS = {
  onAuthenticated: ()=>keycloak && keycloak.authenticated,
  onNotAuthenticated: ()=> keycloak && !keycloak.authenticated,
  onAristocrat: ()=>checkRoles('aristocrat'),
  onDev: ()=>checkRoles('developer'),
  onLibrarian: ()=>checkRoles('librarian'),
}
// ########
// ROUTER #
// ########
export const ROUTER = new Navigo(null, true, '#');

ROUTER
  .on(()=>ROUTER.navigate('librarium')); //Set landing page here!
ROUTER.notFound(()=>{
  const page = ROUTER._lastRouteResolved;
  console.error('Whoopsie! Looks like 404 to me ...', page);
});

// ######
// AUTH #
// ######
const KC_CONF_LOCATION = '../keycloak.json';
const KC_REFRESH_INTERVAL = 5; // seconds -> how often it is checked
const KC_REFRESH_THRESHOLD = 10; // seconds -> remaining time which causes refresh

export let keycloak = null;
let keycloakUpdateInterval = null;

function loadKeycloak(waitForStuff, thenDoStuff) {
  console.debug("Loading keycloak")
  document.querySelector(':root').classList.add('loading');
  if(typeof Keycloak === 'undefined' || !Keycloak){
    axios.get(KC_CONF_LOCATION)
      .then(res => res.data)
      .then(conf => {
        let scriptLocation = `${conf['auth-server-url']}/js/keycloak.js`;
        let scriptNode = document.createElement('script');
        scriptNode.addEventListener('error', errorEvt => {
          console.error('error loading keycloak script', errorEvt)
        });
        scriptNode.addEventListener('load', loadEvt => {
          console.debug('keycloak script loaded!');
          initKeycloak(waitForStuff, thenDoStuff);
        });
        scriptNode.src = scriptLocation;
        scriptNode.async = true;
        document.querySelector('head').appendChild(scriptNode);
      })
      .catch(err => {
        console.error('Fetching Keycloak configuration failed!', err);
      })
  }
}

function initKeycloak(waitForStuff, thenDoStuff){
  if(!keycloak){
    console.debug("Init keycloak")
    // TODO the following seems to be easier than passing the conf object o_O ... we should be able to reuse it!
    keycloak = new Keycloak(KC_CONF_LOCATION);
  }

  keycloak.init({
    onLoad: 'check-sso',
  })
  .success(()=>{
    waitForStuff.then(thenDoStuff);
    updateKeycloakState();
  })
  .error(err => {
    console.error('failed initialising keycloak', err);
  })
}

function updateKeycloakState(){
  if(keycloak && keycloak.authenticated && keycloakUpdateInterval === null){
    keycloakUpdateInterval = setInterval(refreshToken, KC_REFRESH_INTERVAL * 1000)
  } else if(!(keycloak && keycloak.authenticated) && keycloakUpdateInterval !== null){
    clearInterval(keycloakUpdateInterval);
    keycloakUpdateInterval = null;
  }
}

function refreshToken() {
  if(!keycloak) return console.warn("Keycloak, not set");
  keycloak.updateToken(KC_REFRESH_THRESHOLD)
    .success(refreshed => {
      if(refreshed){
        console.debug('keycloak token refreshed');
        updateKeycloakState();
      }
    })
    .error(err => {
        console.err('refreshing token failed:', err);
        updateKeycloakState();
    });
}

function checkRoles(role) {
  return keycloak && keycloak.authenticated && (keycloak.tokenParsed.roles.includes(role) || keycloak.tokenParsed.roles.includes('admin'));
}


// #####
// API #
// #####
export const API = axios.create({
  baseURL: 'http://localhost:8080/v1/',
  timeout: 1000,
  responseType:'json',
});
//inject auth header if not already set and a token is available
API.interceptors.request.use (
  config => {
    if(!config.headers.Authorization && keycloak && keycloak.authenticated){
      config.headers.Authorization = `Bearer ${keycloak.token}`;
    }
    return config;
  },
  error => Promise.reject(error)
);


// #####################
// UI VOODOO FUNCTIONS #
// #####################
function renderPage(loadData, page, args={}) {
  if (!page.template) return ROUTER.navigate('librarium');
  const navPageActive = page.navActive !== undefined ? page.navActive : page.page;
  const root = document.querySelector(':root');
  //loadingScreen
  root.classList.add('loading');
  // query data
  loadData(args).then(data => {
    data = {
      _AUTHENTICATED: (keycloak || {}).authenticated || false,
      ...data,
    };
    // render data to template
    const rendered = Mustache.render(TEMPLATES[page.template], data);
    // generate page element
    let pageElement = document.createElement('div');
    pageElement.classList.add('page');
    pageElement.innerHTML = rendered;
    // store old pages
    const oldPages = document.querySelectorAll('main > .page');
    // ... add class "old" to these
    oldPages.forEach(e => e.classList.add('old'));
    // remove loading screen
    root.classList.remove('loading');
    // add new page to main element
    document.querySelector('main').appendChild(pageElement);
    // update navigation bar (maybe a new item is active now‽)
    NAV_ACTIVE = navPageActive;
    updateNavBar();
    // remove old page elements after woosh animation
    execAfter(()=>oldPages.forEach(e => e.remove()), WHOOSH_DURATION);
  }).catch(e => {
    console.error('we got errœr', e);
    root.classList.remove('loading');
  });
}
PAGE._RENDER = renderPage;

function updateNavBar() {
  let navBarPagesTmp = ALL_PAGES
    .filter(p => p.navPos !== undefined)
    .filter(p => p.conditional())
    .sort((b,a) => b.navPos - a.navPos);
  let navBarPagesLeft = navBarPagesTmp
    .filter(p => p.navPos>=0);
  let navBarPageRight = navBarPagesTmp
    .filter(p => p.navPos<0);

  const newHtml = Mustache.render(TEMPLATES.nav_bar, {
    pagesLeft: NAV_ACTIVE ? navBarPagesLeft.map(p => {
      if (p.page === NAV_ACTIVE) {
        return {...p, class:['active']};
      }
      return p;
    }) : navBarPagesLeft,
    pagesRight: NAV_ACTIVE ? navBarPageRight.map(p => {
      if (p.page === NAV_ACTIVE) {
        return {...p, class:['active']};
      }
      return p;
    }) : navBarPageRight,
  });
  document.querySelector('nav.topnav').outerHTML = newHtml;
}


// #####################
// GENREAL MAGIC STUFF #
// #####################
export const MAGIC = (waitForStuff, thenDoStuff)=>{
  loadKeycloak(waitForStuff, thenDoStuff);
};
