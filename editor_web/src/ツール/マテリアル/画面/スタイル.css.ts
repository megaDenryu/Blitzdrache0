import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../境界/index.ts'

export * from './スタイル/ボタン.css.ts'
export * from './スタイル/層割当.css.ts'

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
    gap: '20px',
    maxWidth: '640px',
})

export const 表題 = style({
    fontSize: '20px',
    fontWeight: 700,
})

export const セクション = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '10px',
})

export const セクション見出し = style({
    fontSize: '14px',
    fontWeight: 700,
    color: エディターCSS変数('テキスト主'),
})

export const 一覧コンテナ = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '6px',
})

export const 行 = style({
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
    padding: '6px 8px',
    backgroundColor: エディターCSS変数('カード背景'),
    border: `1px solid ${エディターCSS変数('カード枠線')}`,
    borderRadius: '4px',
})

export const 名前入力 = style({
    flex: 1,
    padding: '5px 8px',
    fontSize: '12px',
    color: エディターCSS変数('テキスト主'),
    backgroundColor: エディターCSS変数('パネル背景'),
    border: `1px solid ${エディターCSS変数('境界線')}`,
    borderRadius: '3px',
})

export const 色入力 = style({
    width: '32px',
    height: '28px',
    padding: '2px',
    border: `1px solid ${エディターCSS変数('境界線')}`,
    borderRadius: '3px',
    backgroundColor: エディターCSS変数('パネル背景'),
    cursor: 'pointer',
})

export const 色見本 = style({
    width: '20px',
    height: '20px',
    borderRadius: '3px',
    border: `1px solid ${エディターCSS変数('境界線')}`,
    flexShrink: 0,
})
