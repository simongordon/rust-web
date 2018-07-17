
const wasm = require("./main.rs")
wasm.initialize({ noExitRuntime: true }).then(module => {
  // you can call module.cwrap here to get function wrappers for Rust functions

  const add = module.cwrap('add', 'number', ['number', 'number'])
  console.log('Calling rust functions from javascript!')
  console.log(add(1, 2))

  const getData = module.cwrap('get_data', 'string');
  console.log(getData());

  const getHtml = module.cwrap('get_html', 'string');
  // This might usually cause problems if the script tag comes
  // before the tag itself, but by the time this promise is
  // finished it should be here.
  document.getElementById("content").innerHTML = getHtml();
})

