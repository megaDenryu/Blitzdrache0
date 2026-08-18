import { vanillaExtractPlugin } from '@vanilla-extract/vite-plugin'
import { defineConfig } from 'vite'
import path from 'path'

const 編集サーバーの原点 = 'http://127.0.0.1:7901'

// 注意: `strictPort` を立てないと、塞がっていたときViteは黙って隣の番号へずれる。
// どこで開いたか分からなくなるより、起動に失敗して番号の衝突を知らせるほうがよい。
// 開発サーバーの待ち受け口は、GameScriptingTheoryの7801/7802と衝突しない7901/7902を使う。
export default defineConfig({
  plugins: [vanillaExtractPlugin()],
  resolve: {
    alias: {
      // VscodeShellLayout内部の全ファイルが `sengen-ui` という綴りで取り込むため、
      // このアライアス名も揃える（参照: VscodeShellLayout/README.md「利用方法」前提3）。
      'sengen-ui': path.resolve(__dirname, 'submodules/SengenUI'),
      VscodeShellLayout: path.resolve(__dirname, 'submodules/VscodeShellLayout/src/index.ts'),
      SengenThree: path.resolve(__dirname, 'submodules/SengenThree/src/index.ts'),
    },
  },
  server: {
    port: 7902,
    strictPort: true,
    proxy: {
      '/api': 編集サーバーの原点,
    },
  },
  build: {
    outDir: 'dist',
  },
})
