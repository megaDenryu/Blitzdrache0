import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

// タイムラインの1枚のカード(曲全体での小節番号を表示し、選ぶと格子が切り替わる)。
export const カード枠 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '4px',
    minWidth: '96px',
    padding: '6px 10px',
    borderRadius: '4px',
    border: `1px solid ${エディターCSS変数('カード枠線')}`,
    backgroundColor: エディターCSS変数('カード背景'),
    cursor: 'pointer',
    flexShrink: 0,
    selectors: {
        '&[data-節偶奇="1"]': { backgroundColor: エディターCSS変数('カード不透明背景') },
        '&[data-選択中="true"]': {
            borderColor: エディターCSS変数('選択枠線'),
            backgroundColor: エディターCSS変数('選択背景'),
        },
        '&[data-再生中="true"]': { outline: `2px solid ${エディターCSS変数('アクセント文字')}` },
    },
})

export const カードのパターン名 = style({
    fontSize: '12px',
    fontWeight: 600,
    color: エディターCSS変数('テキスト主'),
    selectors: {
        [`${カード枠}[data-選択中="true"] &`]: { color: エディターCSS変数('選択文字') },
    },
})

export const カードの小節番号 = style({
    fontSize: '11px',
    color: エディターCSS変数('テキスト薄'),
    selectors: {
        [`${カード枠}[data-選択中="true"] &`]: { color: エディターCSS変数('選択文字') },
    },
})

export const 繰り返し中の印 = style({
    fontSize: '10px',
    color: エディターCSS変数('アクセント文字'),
    fontWeight: 700,
})
