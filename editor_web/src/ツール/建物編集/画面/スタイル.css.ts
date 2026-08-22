import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../境界/index.ts'

export * from './スタイル/平面図.css.ts'

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
    maxWidth: '720px',
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
})

export const 説明文 = style({
    fontSize: '12px',
    color: エディターCSS変数('テキスト副'),
})

export const 横並び = style({
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
    flexWrap: 'wrap',
})

export const 名前入力 = style({
    flex: 1,
    minWidth: '200px',
    padding: '5px 8px',
    fontSize: '12px',
    color: エディターCSS変数('テキスト主'),
    backgroundColor: エディターCSS変数('パネル背景'),
    border: `1px solid ${エディターCSS変数('境界線')}`,
    borderRadius: '3px',
})

export const 選択ボタン = style({
    padding: '4px 10px',
    fontSize: '12px',
    borderRadius: '3px',
    cursor: 'pointer',
    color: エディターCSS変数('テキスト主'),
    backgroundColor: エディターCSS変数('カード背景'),
    border: `1px solid ${エディターCSS変数('カード枠線')}`,
    selectors: {
        '&[data-selected="true"]': {
            backgroundColor: エディターCSS変数('選択背景'),
            borderColor: エディターCSS変数('選択枠線'),
            color: エディターCSS変数('選択文字'),
        },
    },
})
