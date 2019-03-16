// 'use strict';
import {API, PAGE, keycloak, MAGIC, ROUTER, TEMPLATES, SETUP_NAVBAR} from './base.js';
import './rpgsystems.js';
import './titles.js';
window._PAGE = PAGE;

PAGE('librarium', 'Librarium', 'page_librarium');
PAGE('guilds', 'Gilden', undefined);
PAGE('mybooks', 'Meine Bücher', undefined);
PAGE('aristocracy', 'Aristokratie', 'peaks_of_aristocracy');

SETUP_NAVBAR([
  PAGE.librarium,
  PAGE.guilds,
  PAGE.mybooks,
  PAGE.aristocracy,
]);

/*
 * Authentication
 */
const KC_CONF_LOCATION = '../keycloak.json';
const KC_REFRESH_INTERVAL = 5; // seconds -> how often it is checked
const KC_REFRESH_THRESHOLD = 10; // seconds -> remaining time which causes refresh

const initialLoadingPromise = loadTemplates();

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

// ##########################
// DATA RETRIEVAL FUNCTIONS #
// ##########################


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
  // loadKeycloak();
  MAGIC(initialLoadingPromise, ()=>ROUTER.resolve());
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
