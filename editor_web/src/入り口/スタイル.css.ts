import { style, globalStyle } from '@vanilla-extract/css'
import { エディターCSS変数 } from './テーマ/テーマ変数.ts'

// ビューポート全高を占有し、余白とスクロールバーを抑止するためのグローバル定義。
// line-break: strict は禁則(行頭に句読点等を置かない)を厳格化する基礎設定であり、
// 折返しを許す段落文でも読みやすい位置で折れるようにする(折返し自体の禁止は各ラベルの
// white-space: nowrap が個別に担う)。
globalStyle('html, body', {
    width: '100%',
    height: '100%',
    margin: 0,
    padding: 0,
    overflow: 'hidden',
    backgroundColor: エディターCSS変数('アプリ背景'),
    lineBreak: 'strict',
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
