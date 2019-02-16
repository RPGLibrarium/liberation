/*
 * Resolve router after loading the initial page structure and templates
 */
document.addEventListener("DOMContentLoaded", ()=>{
  loadKeycloak();
});

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

/*
 * Multiple page setup, page routing
 */
const ROUTER = new Navigo(null, true, '#');
//ROUTER.on('*', (a,b,c)=>console.debug(a,b,c)).resolve();
ROUTER
  .on(()=>ROUTER.navigate('librarium'))
  .on('librarium', loadTestpage)
  .on('guilds', ()=>{console.warn("TÜDÜ: guilds")})
  .on('mybooks', ()=>renderPage(loadRpgSystems,TEMPLATES.rpg_systems_list))
  .on('aristocracy', ()=>{console.warn("TÜDÜ: aristocracy")})
  .on('profile', ()=>{console.warn("TÜDÜ: profile")});
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
    loadTpl('rpg_systems_list'),
    loadTpl('titles_list'),
  ])
    .catch(err => console.error('something went wrong (fetching templates)', err));
}

const execAfter = setTimeout;

function renderPage(loadData, template) {
  const root = document.querySelector(':root');
  //loadingScreen
  root.classList.add('loading');
  // query data
  loadData().then(data => {
    // render data to template
    const rendered = Mustache.render(template, data);
    // generate page element
    let page = document.createElement('div');
    page.classList.add('page');
    page.innerHTML = rendered;
    // store old pages
    const oldPages = document.querySelectorAll('main > .page');
    // ... add class "old" to these
    oldPages.forEach(e => e.classList.add('old'));
    // remove loading screen
    root.classList.remove('loading');
    // add new page to main element
    document.querySelector('main').appendChild(page);
    // remove old page elements after woosh animation
    execAfter(()=>oldPages.forEach(e => e.remove()), WHOOSH_DURATION);
  }).catch(e => {
    console.error('we got errœr', e);
    root.classList.remove('loading');
  });
}

function loadRpgSystems() {
  return API({
      method: 'GET',
      url: '/rpgsystems',
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
