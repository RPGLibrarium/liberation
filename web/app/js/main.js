/*
 * Axios, Rest API stuff, HTTP client
 */
const API = axios.create({
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

const TEMPLATES = {};

const PAGES = {};
const _PAGE = (page, title, template, navActive=undefined)=>{
  let obj = {page,title,template};
  if(navActive !== undefined) obj.navActice = navActive;
  PAGES[page] = obj;
}
_PAGE('librarium', 'Librarium', 'page_librarium');
_PAGE('guilds', 'Gilden', undefined);
_PAGE('mybooks', 'Meine Bücher', undefined);
_PAGE('aristocracy', 'Aristokratie', 'peaks_of_aristocracy');
_PAGE('systems', 'Systeme', 'rpg_systems_list', 'librarium');
_PAGE('titles', 'Titel', 'titles_list', 'librarium');
_PAGE('system', 'System', 'rpg_system', 'librarium');
const NAV_BAR_PAGES = [
  PAGES.librarium,
  PAGES.guilds,
  PAGES.mybooks,
  PAGES.aristocracy,
];
let NAV_ACTIVE = 'librarium';

const WHOOSH_DURATION = 1000;

/*
 * Authentication
 */
const KC_CONF_LOCATION = '../keycloak.json';
const KC_REFRESH_INTERVAL = 5; // seconds -> how often it is checked
const KC_REFRESH_THRESHOLD = 10; // seconds -> remaining time which causes refresh

let keycloak = null;
let keycloakUpdateInterval = null;

function loadKeycloak() {
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
          initKeycloak();
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

function initKeycloak(){
  if(!keycloak){
    console.debug("Init keycloak")
    // TODO the following seems to be easier than passing the conf object o_O ... we should be able to reuse it!
    keycloak = new Keycloak(KC_CONF_LOCATION);
  }

  keycloak.init({
    onLoad: 'check-sso',
  })
  .success(()=>{
    initalLoadingPromise.then(()=>ROUTER.resolve());
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

// TODO delete!
const UNLOATh = ()=>document.querySelector(':root').classList.remove('loading');

/*
 * Multiple page setup, page routing
 */
const ROUTER = new Navigo(null, true, '#');
//ROUTER.on('*', (a,b,c)=>console.debug(a,b,c)).resolve();
ROUTER
  .on(()=>ROUTER.navigate('librarium'))
  .on('librarium', ()=>renderPage(()=>Promise.resolve({}),PAGES.librarium))
  .on('guilds', ()=>{console.warn("TÜDÜ: guilds"),UNLOATh()})
  .on('mybooks', ()=>{console.warn("TÜDÜ: mybooks"),UNLOATh()})
  .on('systems', ()=>renderPage(loadRpgSystems,PAGES.systems))
  .on('titles', ()=>renderPage(loadTitles,PAGES.titles))
  .on('aristocracy', ()=>renderPage(()=>Promise.resolve({}),PAGES.aristocracy))
  .on('profile', ()=>{console.warn("TÜDÜ: profile"),UNLOATh()})
  .on('systems/:id', args=>renderPage(loadRpgSystem,PAGES.system, args));
ROUTER.notFound(()=>{
  const page = ROUTER._lastRouteResolved;
  console.error('Whoopsie! Looks like 404 to me ...', page);
});

const initalLoadingPromise = loadTemplates();

function loadTemplates(){
  const loadTpl = name => axios(`templates/${name}.mustache`)
    .then(res => {
      TEMPLATES[name] = res.data;
      Mustache.parse(TEMPLATES[name]);
    });
  return axios.all([
    loadTpl('nav_bar'),
    loadTpl('rpg_systems_list'),
    loadTpl('titles_list'),
    loadTpl('page_librarium'),
    loadTpl('peaks_of_aristocracy'),
    loadTpl('rpg_system'),
  ])
    .catch(err => console.error('something went wrong (fetching templates)', err));
}

const execAfter = setTimeout;

// #####################
// UI VOODOO FUNCTIONS #
// #####################

function renderPage(loadData, page, args={}) {
  const activePage = page.navActice !== undefined ? page.navActice : page.page;
  const root = document.querySelector(':root');
  //loadingScreen
  root.classList.add('loading');
  // query data
  loadData(args).then(data => {
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
    NAV_ACTIVE = activePage;
    updateNavBar();
    // remove old page elements after woosh animation
    execAfter(()=>oldPages.forEach(e => e.remove()), WHOOSH_DURATION);
  }).catch(e => {
    console.error('we got errœr', e);
    root.classList.remove('loading');
  });
}

function updateNavBar() {
  const newHtml = Mustache.render(TEMPLATES.nav_bar, {
    pages: NAV_ACTIVE ? NAV_BAR_PAGES.map(p => {
      if (p.page === NAV_ACTIVE) {
        return {...p, class:['active']};
      }
      return p;
    }) : NAV_BAR_PAGES,
    auth: keycloak.authenticated,
  });
  document.querySelector('nav.topnav').outerHTML = newHtml;
}


// ##########################
// DATA RETRIEVAL FUNCTIONS #
// ##########################

function loadRpgSystems() {
  return API({
      method: 'GET',
      url: '/rpgsystems',
  }).then(stuff => stuff.data);
}
function loadRpgSystem(args) {
  return API({
      method: 'GET',
      url: '/rpgsystems/' + encodeURIComponent(args.id),
  }).then(stuff => stuff.data);
}
function loadTitles() {
  return API({
      method: 'GET',
      url: '/titles',
  }).then(stuff => stuff.data);
}

function loadTestpage(){
  // rpg systems
  API({
      method: 'GET',
      url: '/rpgsystems',
  })
    .then(stuff => {
      let rendered = Mustache.render(TEMPLATES.rpg_systems_list, stuff.data);
      let section = document.createElement('section');
      section.classList.add('content');
      section.innerHTML = rendered;
      document.querySelector('main').appendChild(section);
    })
    .catch(err => console.error('we got error'));

    // titles
    API({
        method: 'GET',
        url: '/titles',
    })
      .then(stuff => {
        let rendered = Mustache.render(TEMPLATES.titles_list, stuff.data);
        let section = document.createElement('section');
        section.classList.add('content');
        section.innerHTML = rendered;
        document.querySelector('main').appendChild(section);
      })
      .catch(err => console.error('we got error'));
}


// #####################
// ADD EVENT LISTENERS #
// #####################

/*
 * Resolve router after loading the initial page structure and templates
 */
document.addEventListener("DOMContentLoaded", ()=>{
  loadKeycloak();
});

document.querySelector(':root').addEventListener('click', e=>{
  if(e.target.id === 'navLogin'){
    e.preventDefault();
    console.info('You pretend to belong to us? Prove it!');
    keycloak.login();
    return;
  }
  if(e.target.matches('.systems tr[data-rpgsystemid] td *, .systems tr[data-rpgsystemid] td')){
    let node = e.target;
    while(!node.hasAttribute('data-rpgsystemid')){
      node = node.parentNode;
    }
    let systemid = node.getAttribute('data-rpgsystemid');
    ROUTER.navigate('systems/' + encodeURIComponent(systemid));
    return;
  }
});
