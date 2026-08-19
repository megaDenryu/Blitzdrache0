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
    boxShadow: エディターCSS変数('カード影'),
})

export const 見出し = style({
    fontSize: '11px',
    fontWeight: 600,
    color: エディターCSS変数('テキスト副'),
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
})

export const 材質グリッド = style({
    display: 'grid',
    gridTemplateColumns: 'repeat(4, 1fr)',
    gap: '4px',
})

export const 材質ボタン = style({
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    padding: '6px 2px',
    fontSize: '10px',
    fontWeight: 500,
    borderRadius: '4px',
    border: `1px solid ${エディターCSS変数('ボタン枠線')}`,
    backgroundColor: エディターCSS変数('ボタン背景'),
    color: エディターCSS変数('テキスト副'),
    cursor: 'pointer',
    transition: 'all 0.15s ease',
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    maxWidth: '100%',
    ':hover': {
        backgroundColor: エディターCSS変数('ボタンホバー背景'),
    },
})

globalStyle(`${材質ボタン}[data-selected="true"]`, {
    backgroundColor: エディターCSS変数('選択背景'),
    borderColor: エディターCSS変数('選択枠線'),
    color: エディターCSS変数('選択文字'),
})

export const ベイク区画 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '6px',
    paddingTop: '8px',
    borderTop: `1px solid ${エディターCSS変数('境界線')}`,
})

export const アクションボタン = style({
    padding: '6px 8px',
    fontSize: '11px',
    fontWeight: 500,
    borderRadius: '4px',
    border: `1px solid ${エディターCSS変数('ボタン枠線')}`,
    backgroundColor: エディターCSS変数('ボタン背景'),
    color: エディターCSS変数('テキスト主'),
    cursor: 'pointer',
    transition: 'all 0.15s ease',
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    ':hover': {
        backgroundColor: エディターCSS変数('ボタンホバー背景'),
    },
})
