function setupDiagrams() {
  if (typeof mermaid === 'undefined') {
    return
  }

  mermaid.initialize({ startOnLoad: false })
  const e = document.querySelectorAll("code.language-mermaid");
  let n = 0;
  for (const t of e) {
    const e = `mermaid${n}`;
    n++;
    const o = (e) => { t.innerHTML = e }
    const d = t.textContent;
    mermaid.mermaidAPI.render(e, d, o)
  }
}

function setupMath() {
  if (typeof katex === 'undefined') {
    return
  }

  for (const e of document.querySelectorAll(".language-math")) {
    katex.render(e.textContent, e)
  }
}

function setupHighlight() {
  if (typeof hljs === 'undefined') {
    return
  }

  hljs.highlightAll();
}

function setup() {
  setupMath()
  setupDiagrams()
  setupHighlight()
}

function setupWebSockets() {
  if (typeof ReconnectingWebSocket === 'undefined') {
    return
  }

  var webSocketUrl = 'ws://' + window.location.host;

  var socket = new ReconnectingWebSocket(webSocketUrl);
  socket.maxReconnectInterval = 5000;

  socket.onmessage = event => {
    document.getElementById('root').innerHTML = event.data;
    setup()
  }

  socket.onclose = () => {
    // Close the browser window.
    window.open('', '_self', '');
    window.close();
  }
}

document.addEventListener('DOMContentLoaded', () => {
  setup()
  setupWebSockets()
});
