const native = require('./index.node');

window.addEventListener('DOMContentLoaded', () => {
  document.getElementById('greeting').innerText = native.hello();
})

window.crash = function() {
  try {
    console.log("Calling native.crash()");
    native.crash();
  } catch (e) {
    console.log("It crashed:\n", e, "\n\nStack:\n", e.stack);
  }
}
