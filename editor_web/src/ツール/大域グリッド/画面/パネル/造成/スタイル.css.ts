import { style, globalStyle } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

export const パネル = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '10px',
    padding: '12px',
    backgroundColor: エディターCSS変数('カード背景'),
    backdropFilter: エディターCSS変数('ガラス背景ぼかし'),
    borderRadius: '8px',
    border: `1px solid ${エディターCSS変数('カード枠線')}`,
})

export const 見出し行 = style({
    display: 'flex',
    justifyContent: 'space-between',
    fontSize: '11px',
    fontWeight: 600,
    color: エディターCSS変数('テキスト副'),
})

export const 補助テキスト = style({
    color: エディターCSS変数('テキストコード'),
    fontSize: '10px',
})

export const ボタングループ = style({
    display: 'grid',
    gridTemplateColumns: 'repeat(3, 1fr)',
    gap: '4px',
})

export const ブラシ種別ボタン = style({
    padding: '6px 4px',
    fontSize: '11px',
    borderRadius: '4px',
    border: `1px solid ${エディターCSS変数('ボタン枠線')}`,
    backgroundColor: エディターCSS変数('ボタン背景'),
    color: エディターCSS変数('テキスト副'),
    cursor: 'pointer',
    transition: 'all 0.15s ease',
    ':hover': {
        backgroundColor: エディターCSS変数('ボタンホバー背景'),
    },
})

globalStyle(`${ブラシ種別ボタン}[data-selected="true"]`, {
    backgroundColor: エディターCSS変数('選択背景'),
    borderColor: エディターCSS変数('選択枠線'),
    color: エディターCSS変数('選択文字'),
})
