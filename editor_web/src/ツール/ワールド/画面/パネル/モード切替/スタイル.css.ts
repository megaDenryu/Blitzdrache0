import { style, globalStyle } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

export const コンテナ = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
})

export const グリッド = style({
    display: 'grid',
    // 3列だと右サイドバー幅では1セルの余白が足りず、日本語ラベルが任意の位置で
    // 折り返される(「大域カメ/ラ」の分断と同種)。2列にしてセル幅を確保する。
    gridTemplateColumns: 'repeat(2, 1fr)',
    gap: '4px',
    padding: '4px',
    backgroundColor: エディターCSS変数('カード不透明背景'),
    borderRadius: '8px',
    border: `1px solid ${エディターCSS変数('境界線')}`,
})

export const モードボタン = style({
    padding: '6px 4px',
    fontSize: '11px',
    fontWeight: 500,
    borderRadius: '4px',
    border: 'none',
    cursor: 'pointer',
    backgroundColor: 'transparent',
    color: エディターCSS変数('テキスト薄'),
    transition: 'all 0.15s ease',
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

// ヒント文は複数語からなる操作案内であり折返しを許す。text-wrap: prettyで折り位置を
// 最適化する(グローバルのline-break: strictと組み合わせ、禁則も守った読みやすい折返しにする)。
export const ヒント枠 = style({
    padding: '8px 12px',
    fontSize: '11px',
    lineHeight: '1.4',
    backgroundColor: エディターCSS変数('カード不透明背景'),
    borderRadius: '8px',
    border: `1px solid ${エディターCSS変数('境界線')}`,
    color: エディターCSS変数('テキスト薄'),
    textWrap: 'pretty',
})
