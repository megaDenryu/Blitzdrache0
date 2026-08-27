import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../境界/index.ts'

export * from './スタイル/進行の帯.css.ts'
export * from './スタイル/トラック.css.ts'
export * from './スタイル/格子.css.ts'

export const コンテナ = style({
    width: '100%',
    height: '100%',
    overflowY: 'auto',
    boxSizing: 'border-box',
    padding: '16px 24px',
    backgroundColor: エディターCSS変数('ビューポート背景'),
    color: エディターCSS変数('テキスト主'),
    display: 'flex',
    flexDirection: 'column',
    gap: '16px',
})

export const 本文幅 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '16px',
    width: '100%',
    maxWidth: '1200px',
})

export const ヘッダー行 = style({
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    flexWrap: 'wrap',
    gap: '12px',
    paddingBottom: '12px',
    borderBottom: `1px solid ${エディターCSS変数('境界線')}`,
})

export const タイトル = style({
    fontSize: '18px',
    fontWeight: 700,
    color: エディターCSS変数('テキスト主'),
})

export const 情報バッジ群 = style({
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
    flexWrap: 'wrap',
})

export const 情報バッジ = style({
    display: 'inline-flex',
    alignItems: 'center',
    gap: '4px',
    padding: '3px 8px',
    fontSize: '12px',
    borderRadius: '4px',
    backgroundColor: エディターCSS変数('カード背景'),
    border: `1px solid ${エディターCSS変数('カード枠線')}`,
    color: エディターCSS変数('テキスト副'),
})

export const エディター領域 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '12px',
    overflowX: 'auto',
    paddingBottom: '8px',
})
