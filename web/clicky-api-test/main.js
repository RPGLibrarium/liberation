document.addEventListener("DOMContentLoaded", initPage);

const API = axios.create({
    baseURL: 'http://localhost:8080/',
    timeout: 1000,
});

const _NODES = {};

function initPage(){
    _NODES.actions = document.querySelector("#actions");
    _NODES.result = document.querySelector("#result");
    _NODES.chkVerbose = document.querySelector("#chkVerbose");

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
}

function displayResult(data) {
    console.log('Result:', data);
    if(!_NODES.chkVerbose.checked){
        data = data.data || data;
    }
    _NODES.result.innerText = JSON.stringify(data, null, 2);
}
function displayError(err) {
    console.log('Error:', err);
    _NODES.result.innerText = `Whoopsie ...\n${err}`;
}

function showDialogForm(elem, fnOk, fnNope){
    let method = elem.getAttribute('data-method');
    let path = elem.getAttribute('data-path');

    let dataInputs = JSON.parse(elem.getAttribute('data-inputs') || '[]');

    let message = `${method} ${path} <br/> Enter data plz!`;
    let inputs = [];

    for(let inp of dataInputs) {
        if((inp.type || 'text') === 'text'){
            let inpId = `_dialog_input_${inp.name}`;
            let inputEl = document.createElement('input');
            inputEl.id = inpId;
            inputEl.type = inp.type || 'text';
            if(inp.required){
                inp.required = true;
            }
            inputEl.placeholder = inp.placeholder || inp.name;
            inputEl.setAttribute('data-name', inp.name);

            let label = document.createElement('label');
            label.innerText = `${inp.name}:`
            label.setAttribute('for', inpId);

            inputs.push(label);
            inputs.push(inputEl);
        }
    }
    console.log(dataInputs, inputs);

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
            fnOk(data)
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
        .catch(err => displayError(err))
        .then(stuff => displayResult(stuff));
}
function doInputAction(elem) {
    showDialogForm(elem, inputResult => {
        console.log('input result:', inputResult);
    });
}
