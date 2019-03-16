let PAGES = {};
const _PAGE = (page, title, template, navActive=undefined)=>{
  let obj = {page,title,template};
  if(navActive !== undefined) obj.navActice = navActive;
  PAGES[page] = obj;
};
PAGES = _PAGE;

export default _PAGE;
