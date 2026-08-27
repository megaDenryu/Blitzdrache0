import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../../境界/index.ts'

export const パネル外枠 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '12px',
    padding: '16px',
    backgroundColor: エディターCSS変数('カード背景'),
    border: `1px solid ${エディターCSS変数('カード枠線')}`,
    borderRadius: '6px',
    boxSizing: 'border-box',
    width: '100%',
})

export const パネル見出し = style({
    fontSize: '13px',
    fontWeight: 700,
    color: エディターCSS変数('テキスト主'),
    paddingBottom: '6px',
    borderBottom: `1px solid ${エディターCSS変数('境界線')}`,
})

export const 区画 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
})

export const 区画見出し = style({
    fontSize: '12px',
    fontWeight: 600,
    color: エディターCSS変数('テキスト副'),
})

export const 行コンテナ = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '4px',
})

export const 横並び行 = style({
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
    flexWrap: 'wrap',
})

export const 項目ラベル = style({
    fontSize: '11px',
    color: エディターCSS変数('テキスト薄'),
})
