const KC_CONF_LOCATION = '../keycloak.json';
const KC_REFRESH_INTERVAL = 5; // seconds -> how often it is checked
const KC_REFRESH_THRESHOLD = 10; // seconds -> remaining time which causes refresh

const _NODES = {};
let keycloakConf = null;
let keycloak = null;
let keycloakUpdateInterval = null;

const API = axios.create({
    baseURL: 'http://localhost:8080/',
    timeout: 1000,
    responseType:'json',
});
// inject auth header if not already set and a token is available
API.interceptors.request.use (
  config => {
    if(!config.headers.Authorization && keycloak && keycloak.authenticated){
      config.headers.Authorization = `Bearer ${keycloak.token}`;
    }
    return config;
  },
  error => Promise.reject(error)
);


document.addEventListener("DOMContentLoaded", initPage);

function initPage(){
    _NODES._HEAD = document.querySelector('head');
    _NODES.actions = document.querySelector("#actions");
    _NODES.result = document.querySelector("#result");
    _NODES.chkVerbose = document.querySelector("#chkVerbose");
    _NODES.keycloak = document.querySelector('#keycloak');
    _NODES.loadKeycloak = document.querySelector("#loadKeycloak");
    _NODES.kcLogin = document.querySelector("#kcLogin");
    _NODES.kcLogout = document.querySelector("#kcLogout");
    _NODES.kcTokenInfo = document.querySelector("#kcTokenInfo");

    // check for existing keycloak data in URL -> init keycloak if exists
    if(location.hash && location.hash.length > 1){
      let fakeQuery = location.hash.replace(/^#/, '?');
      let params = new URL(fakeQuery, location).searchParams;
      if(params.has('state') && params.has('session_state') && params.has('code')){
        loadKeycloak();
      }
    }

    _NODES.actions.addEventListener("click", evt => {
        window._evt = evt;
        // console.log('clicked!', evt);
        let target = evt.target;
        let classes = target.classList;
        if(!classes.contains('action')) return;
        if(classes.contains('simple')){
            doSimpleAction(target);
        }if(classes.contains('input')){
            doInputAction(target);
        }
    });

    _NODES.loadKeycloak.addEventListener('click', loadKeycloak);
    _NODES.kcLogin.addEventListener('click', kcLogin);
    _NODES.kcLogout.addEventListener('click', kcLogout);
    _NODES.kcTokenInfo.addEventListener('click', kcTokenInfo);
}

function loadKeycloak() {
  if(!keycloakConf){
    axios.get(KC_CONF_LOCATION)
      .then(res => {
        keycloakConf = keycloakConf || res.data;
        loadKeycloak();
      })
      .catch(err => {
        displayError(err);
        vex.dialog.alert('Fetching Keycloak configuration failed!');
      })
      return;
  }
  if(typeof Keycloak === 'undefined' || !Keycloak){
    let scriptLocation = `${keycloakConf['auth-server-url']}/js/keycloak.js`;
    let scriptNode = document.createElement('script');
    scriptNode.addEventListener('error', errorEvt => {
      console.error('error loading keycloak script', errorEvt)
      vex.dialog.alert('Loading Keycloak library failed!');
    });
    scriptNode.addEventListener('load', loadEvt => {
      console.debug('keycloak script loaded!');
      loadKeycloak();
    });
    scriptNode.src = scriptLocation;
    scriptNode.async = true;
    _NODES._HEAD.appendChild(scriptNode);
    return;
  }
  //_NODES.loadKeycloak.style.display = 'none';
  _NODES.keycloak.setAttribute('data-init', 'true');
  initKeycloak();
}

function initKeycloak(){
  if(!keycloak){
    // TODO the following seems to be easier than passing the conf object o_O ... we should be able to reuse it!
    keycloak = new Keycloak(KC_CONF_LOCATION);
  }
  keycloak.init({
    onLoad: 'check-sso',
  })
    .then(updateKeycloakState)
    .catch(err => {
      console.error('failed initialising keycloak', err);
      vex.dialog.alert('Keycloak initialisation failed!')
    })
}

function updateKeycloakState(){
  let auth = keycloak.authenticated;
  if(auth && keycloakUpdateInterval === null){
    keycloakUpdateInterval = setInterval(refreshToken, KC_REFRESH_INTERVAL * 1000)
  }else if(!auth && keycloakUpdateInterval !== null){
    clearInterval(keycloakUpdateInterval);
    keycloakUpdateInterval = null;
  }
  _NODES.keycloak.setAttribute('data-auth', auth);
}

function refreshToken() {
  if(keycloak){
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
          vex.dialog.show('Refreshing Keycloak token failed!');
      });
  }
}

function kcLogin(){
  if(keycloak){
    keycloak.login();
  }
}

function kcLogout(){
  if(keycloak){
    keycloak.logout();
  }
}

function kcTokenInfo(){
  if(keycloak){
    displayResult(keycloak.tokenParsed, false);
  }
}

function displayResult(data, printToConsole = true){
    printToConsole && console.log('Result:', data);
    if(!_NODES.chkVerbose.checked){
        data = data.data || data;
    }
    _NODES.result.innerText = JSON.stringify(data, null, 2);
}
function displayError(err, printToConsole = true){
    printToConsole && console.log('Error:', err);
    _NODES.result.innerText = `Whoopsie ...\n${err}`;
}

function showDialogForm(elem, fnOk, fnNope){
    let method = elem.getAttribute('data-method');
    let path = elem.getAttribute('data-path');

    let dataInputs = JSON.parse(elem.getAttribute('data-inputs') || '[]');

    let message = `${method} ${path} <br/> Enter data plz!`;
    let inputs = [];
    let inputsByName = {};

    for(let inp of dataInputs) {
        if(inp.name) { inputsByName[inp.name] = inp; }
        if(['text', 'number'].includes(inp.type || 'text')){
            let inpId = `_dialog_input_${inp.name}`;
            let inputEl = document.createElement('input');
            inputEl.id = inpId;
            inputEl.type = inp.type || 'text';
            if(inp.required !== false){
                inputEl.required = true;
            }
            inputEl.placeholder = inp.placeholder || inp.name;
            inputEl.name = inp.name;
            inputEl.setAttribute('data-name', inp.name);

            let label = document.createElement('label');
            label.innerText = `${inp.name}:`
            label.setAttribute('for', inpId);

            inputs.push(label);
            inputs.push(inputEl);
        }
    }
    console.log(dataInputs, inputs);

    let fixData = (data) => {
      for(let key of Object.keys(data)){
        let meta = inputsByName[key];
        if(!meta) continue;
        if(meta.type === 'number'){
          data[key] = Number(data[key]);
        }
      }
      return data;
    }

    vex.dialog.open({
        unsafeMessage: message,
        //input: [
        //    '<input name="username" type="text" placeholder="Username" required />',
        //    '<input name="password" type="password" placeholder="Password" required />'
        //].join(''),
        input: inputs.map(el => el.outerHTML).join(''),
        buttons: [
            {...vex.dialog.buttons.YES, text: 'do it!' },
            {...vex.dialog.buttons.NO, text: 'changed my mind' }
        ],
        callback: data => {
          if(data)
            fnOk(fixData(data))
          else
            fnNope()
        }
    });
}

function doSimpleAction(elem) {
    let method = elem.getAttribute('data-method');
    let path = elem.getAttribute('data-path');

    window._req =
    API({
        method: method,
        url: path,
    })
        .then(stuff => displayResult(stuff))
        .catch(err => displayError(err));
}
function doInputAction(elem) {
    showDialogForm(elem, inputResult => {
        console.log('input result:', inputResult);

        let method = elem.getAttribute('data-method');
        let path = elem.getAttribute('data-path');
        let property = elem.getAttribute('data-property');
        let data = inputResult;
        if(property){
          data = { [property]: data };
        }

        window._req =
        API({
            method: method,
            url: path,
            data: data,
        })
            .then(stuff => displayResult(stuff))
            .catch(err => displayError(err));
    }, ()=>{});
}
