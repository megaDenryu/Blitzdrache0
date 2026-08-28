import { style, globalStyle } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

// モードのボタンはエディタ領域の上部の固定の行へ横一列で並ぶ。いまどのモードかが分からないと
// 左ボタンの意味が読めないため、この並びはスクロールで消えない位置に置く(設計正本の判断14)。
export const モードの並び = style({
    display: 'flex',
    alignItems: 'center',
    gap: '4px',
    padding: '3px',
    backgroundColor: エディターCSS変数('カード不透明背景'),
    borderRadius: '8px',
    border: `1px solid ${エディターCSS変数('境界線')}`,
})

export const モードボタン = style({
    padding: '5px 10px',
    fontSize: '11px',
    fontWeight: 500,
    borderRadius: '4px',
    border: '1px solid transparent',
    cursor: 'pointer',
    backgroundColor: 'transparent',
    color: エディターCSS変数('テキスト薄'),
    transition: 'all 0.15s ease',
    whiteSpace: 'nowrap',
    ':hover': {
        color: エディターCSS変数('テキスト主'),
        backgroundColor: エディターCSS変数('ボタン背景'),
    },
})

// 活性表現はエクスプローラーの選択項目・造成ブラシの選択ボタンと同じ淡若葉地+糸杉文字+
// 糸杉の細枠に揃える(工房テーマの活性表現の統一)。
globalStyle(`${モードボタン}[data-selected="true"]`, {
    backgroundColor: エディターCSS変数('選択背景'),
    borderColor: エディターCSS変数('選択枠線'),
    color: エディターCSS変数('選択文字'),
})
