// const js = import("./node_modules/@ohtsukajapan/hello-wasm/hello_wasm.js");
const js = import("../pkg/hello_wasm.js");
// const js = import("../wasm/hello_wasm.js");
js.then(js => {
  js.greet("WebAssembly");
  
  let array_nums = [1,2,3,4,5,6,7,8,9];
  js.array_js2rust(array_nums);

  console.log("finish");
});