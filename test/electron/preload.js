const native = require('./index.node');

window.neonc = native;

window.addEventListener('DOMContentLoaded', () => {
  document.getElementById('greeting').innerText = native.hello();
})

window.crash = function() {
  try {
    console.log("Calling native.crash()");
    native.crash_deep();
  } catch (e) {
    console.log("It crashed:\n", e, "\n\nStack:\n", e.stack);
  }
}
