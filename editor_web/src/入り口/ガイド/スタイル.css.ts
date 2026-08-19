import { style, globalStyle } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../境界/index.ts'

export const コンテナ = style({
    width: '100%',
    height: '100%',
    overflowY: 'auto',
    boxSizing: 'border-box',
    padding: '24px 32px',
    backgroundColor: エディターCSS変数('ビューポート背景'),
    color: エディターCSS変数('テキスト主'),
})

export const 本文幅 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '28px',
    maxWidth: '720px',
})

export const 表題 = style({
    fontSize: '20px',
    fontWeight: 700,
})

export const セクション = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
})

export const セクション見出し = style({
    fontSize: '14px',
    fontWeight: 700,
    color: エディターCSS変数('テキスト主'),
})

export const 段落 = style({
    margin: 0,
    fontSize: '13px',
    lineHeight: '1.7',
    color: エディターCSS変数('テキスト副'),
})

export const 手順リスト = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '4px',
    fontSize: '13px',
    lineHeight: '1.7',
    color: エディターCSS変数('テキスト副'),
    paddingLeft: '4px',
})

// p要素の既定の上下マージンが手順リストの行間へ二重に効くため打ち消す。
globalStyle(`${手順リスト} p`, {
    margin: 0,
})

export const コード欄 = style({
    fontFamily: 'monospace',
    fontSize: '12px',
    color: エディターCSS変数('テキストコード'),
    backgroundColor: エディターCSS変数('カード不透明背景'),
    border: `1px solid ${エディターCSS変数('境界線')}`,
    borderRadius: '4px',
    padding: '8px 10px',
    overflowX: 'auto',
    whiteSpace: 'pre',
})
