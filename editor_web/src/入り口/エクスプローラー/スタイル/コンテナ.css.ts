import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../テーマ/テーマ変数.ts'

// 左サイドバーのエクスプローラー木ビューの外枠と見出しのスタイル。
// テーマCSS変数により各配色トークンを参照し、WCAG AA(4.5:1以上)のコントラスト比を保証する。
export const コンテナ = style({
    display: 'flex',
    flexDirection: 'column',
    width: '100%',
    height: '100%',
    padding: '8px 0',
    overflowY: 'auto',
    userSelect: 'none',
    boxSizing: 'border-box',
    backgroundColor: エディターCSS変数('サイドバー背景'),
})

export const セクション見出し = style({
    fontSize: '11px',
    fontWeight: 'bold',
    textTransform: 'uppercase',
    color: エディターCSS変数('テキスト薄'),
    padding: '6px 12px 2px 12px',
    letterSpacing: '0.5px',
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
})
