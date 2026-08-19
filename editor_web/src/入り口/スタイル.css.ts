import { style, globalStyle } from '@vanilla-extract/css'
import { エディターCSS変数 } from './テーマ/テーマ変数.ts'

// ビューポート全高を占有し、余白とスクロールバーを抑止するためのグローバル定義。
globalStyle('html, body', {
    width: '100%',
    height: '100%',
    margin: 0,
    padding: 0,
    overflow: 'hidden',
    backgroundColor: エディターCSS変数('アプリ背景'),
})

globalStyle('#app', {
    width: '100%',
    height: '100%',
    overflow: 'hidden',
})

export const 外殻ルート = style({
    width: '100%',
    height: '100%',
    position: 'relative',
    overflow: 'hidden',
})
