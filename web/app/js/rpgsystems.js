import {API,PAGE,ROUTER} from './base.js';

PAGE('systems', 'Systeme', 'rpg_systems_list', 'librarium');
PAGE('system', 'System', 'rpg_system', 'librarium');

ROUTER
  .on('systems', ()=>PAGE._RENDER(loadRpgSystems,PAGE.systems))
  .on('systems/:id', args=>PAGE._RENDER(loadRpgSystem,PAGE.system, args));

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
