const API = axios.create({
    baseURL: 'http://localhost:8080/v1/',
    timeout: 1000,
    responseType:'json',
});
// inject auth header if not already set and a token is available
// API.interceptors.request.use (
//   config => {
//     if(!config.headers.Authorization && keycloak && keycloak.authenticated){
//       config.headers.Authorization = `Bearer ${keycloak.token}`;
//     }
//     return config;
//   },
//   error => Promise.reject(error)
// );

const TEMPLATES = {};

// testing in progrss
document.addEventListener("DOMContentLoaded", initPage);

function initPage(){
  loadTemplates().then(()=>loadStuff());
  // loadStuff();
}

function loadTemplates(){
  return axios('templates/rpg_systems_list.mustache')
    .then(res => {
      TEMPLATES.rpg_systems_list = res.data;
      Mustache.parse(TEMPLATES.rpg_systems_list);
    })
    .catch(err => console.error('something went wrong (fetching templates)', err));
}

function loadStuff(){
  API({
      method: 'GET',
      url: '/rpgsystems',
  })
      .then(stuff => {
        let rendered = Mustache.render(TEMPLATES.rpg_systems_list, stuff.data);
        console.log('rendered', rendered);
        let section = document.createElement('section');
        section.classList.add('content');
        section.innerHTML = rendered;
        document.querySelector('main').appendChild(section);
      })
      .catch(err => console.error('we got error'));
}
