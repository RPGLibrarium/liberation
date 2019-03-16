import {API,PAGE,ROUTER} from './base.js';

PAGE('titles', 'Titel', 'titles_list', 'librarium');

ROUTER
  .on('titles', ()=>PAGE._RENDER(loadTitles,PAGE.titles));

function loadTitles() {
  return API({
      method: 'GET',
      url: '/titles',
  }).then(stuff => stuff.data);
}
