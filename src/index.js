
const wasm = require("./main.rs")
wasm.initialize({ noExitRuntime: true }).then(module => {
  // you can call module.cwrap here to get function wrappers for Rust functions

  const add = module.cwrap('add', 'number', ['number', 'number'])
  console.log('Calling rust functions from javascript!')
  console.log(add(1, 2))

  const getData = module.cwrap('get_data', 'string');
  console.log(getData());
})

