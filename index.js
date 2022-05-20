// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const rust = import("./pkg/concrete_wasm_test.js");

rust.then((m) => m.generate()).catch(console.error);
