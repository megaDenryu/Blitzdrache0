import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../テーマ/テーマ変数.ts'

// 設定・テーマ選択パネルの外枠と見出しのスタイル。
// 全てエディターテーマCSS変数を参照し、直書き色を持たない。
export const コンテナ = style({
    display: 'flex',
    flexDirection: 'column',
    width: '100%',
    height: '100%',
    padding: '12px',
    overflowY: 'auto',
    userSelect: 'none',
    boxSizing: 'border-box',
    backgroundColor: エディターCSS変数('サイドバー背景'),
    color: エディターCSS変数('テキスト主'),
    gap: '16px',
})

export const セクション見出し = style({
    fontSize: '11px',
    fontWeight: 'bold',
    textTransform: 'uppercase',
    color: エディターCSS変数('テキスト薄'),
    letterSpacing: '0.5px',
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
})
