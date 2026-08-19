import { style, globalStyle } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

export const コンテナ = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
})

export const グリッド = style({
    display: 'grid',
    // 4列だと右サイドバー幅では1セルの余白が足りず、日本語ラベルが任意の位置で
    // 折り返される(「大域カメ/ラ」の分断の原因)。2列にしてセル幅を確保する。
    gridTemplateColumns: 'repeat(2, 1fr)',
    gap: '4px',
    padding: '4px',
    backgroundColor: エディターCSS変数('カード不透明背景'),
    borderRadius: '8px',
    border: `1px solid ${エディターCSS変数('境界線')}`,
})

export const モードボタン = style({
    padding: '6px 2px',
    fontSize: '10px',
    fontWeight: 500,
    borderRadius: '4px',
    border: 'none',
    cursor: 'pointer',
    backgroundColor: 'transparent',
    color: エディターCSS変数('テキスト薄'),
    transition: 'all 0.15s ease',
    textAlign: 'center',
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    ':hover': {
        color: エディターCSS変数('テキスト主'),
        backgroundColor: エディターCSS変数('ボタン背景'),
    },
})

globalStyle(`${モードボタン}[data-selected="true"]`, {
    backgroundColor: エディターCSS変数('アクセント背景'),
    color: エディターCSS変数('アクセント文字白'),
})
