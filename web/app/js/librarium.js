import {API,PAGE,ROUTER} from './base.js';

PAGE('librarium', 'Librarium', undefined, 0);

ROUTER
  .on('librarium', ()=>{console.debug("Here");
    ROUTER.navigate('systems')});
