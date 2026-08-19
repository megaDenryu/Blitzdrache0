// シーケンサー用の最小 HTTP サーバー（依存ゼロ、Node 組み込みのみ）
// 起動: node server.js  →  http://localhost:7901 で HTML を開く
// GET  /state     現在の状態（JSON）を返す。ページはこれを自動ポーリングして適用する
// POST /state     状態を受け取り state.json へ保存する。AI はここへ投げて楽曲を打ち込む
const http = require('http');
const fs = require('fs');
const path = require('path');

const port = 7901;
const htmlFile = path.join(__dirname, 'シーケンサー.html');
const stateFile = path.join(__dirname, 'state.json');

let state = { progKey: 'town', bpm: 110, insts: [], tracks: [], trackVols: [], mix: {} };
try {
  Object.assign(state, JSON.parse(fs.readFileSync(stateFile, 'utf8')));
} catch (e) {
  // 初回起動は空の状態でよい
}

const server = http.createServer((req, res) => {
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'GET, POST, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type');

  if (req.method === 'OPTIONS') {
    res.writeHead(204);
    res.end();
    return;
  }

  const url = new URL(req.url, 'http://localhost');

  if (url.pathname === '/state') {
    if (req.method === 'GET') {
      res.writeHead(200, { 'Content-Type': 'application/json; charset=utf-8' });
      res.end(JSON.stringify(state));
      return;
    }
    if (req.method === 'POST') {
      let body = '';
      req.on('data', (chunk) => { body += chunk; });
      req.on('end', () => {
        try {
          const data = JSON.parse(body);
          Object.assign(state, data);
          fs.writeFileSync(stateFile, JSON.stringify(state, null, 2));
          res.writeHead(200, { 'Content-Type': 'application/json; charset=utf-8' });
          res.end(JSON.stringify({ ok: true }));
        } catch (e) {
          res.writeHead(400, { 'Content-Type': 'text/plain; charset=utf-8' });
          res.end('bad json: ' + e.message);
        }
      });
      return;
    }
  }

  if (url.pathname === '/' || url.pathname === '/index.html') {
    fs.readFile(htmlFile, (err, data) => {
      if (err) {
        res.writeHead(500, { 'Content-Type': 'text/plain; charset=utf-8' });
        res.end('HTML が見つかりません');
        return;
      }
      res.writeHead(200, { 'Content-Type': 'text/html; charset=utf-8' });
      res.end(data);
    });
    return;
  }

  res.writeHead(404, { 'Content-Type': 'text/plain; charset=utf-8' });
  res.end('not found');
});

server.listen(port, () => {
  console.log('http://localhost:' + port + '  (Ctrl+C で停止)');
});