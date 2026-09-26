import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'
import { 横並び行 } from '../共通/スタイル/枠と見出し.css.ts'

// コード進行パネルに固有の配置。外枠・見出し・入力欄・ボタン・帯の見た目は共通のスタイルが持つ。
export const 一覧枠 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '6px',
})

export const 既定進行行枠 = style({
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    gap: '8px',
    padding: '6px 10px',
    backgroundColor: エディターCSS変数('ビューポート背景'),
    border: `1px solid ${エディターCSS変数('境界線')}`,
    borderRadius: '4px',
    fontSize: '12px',
})

export const 進行名 = style({
    fontWeight: 600,
    color: エディターCSS変数('テキスト主'),
})

export const 和音要約 = style({
    fontSize: '11px',
    color: エディターCSS変数('テキストコード'),
    fontFamily: 'monospace',
})

export const 独自進行行枠 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '4px',
    padding: '8px 10px',
    backgroundColor: エディターCSS変数('ビューポート背景'),
    border: `1px solid ${エディターCSS変数('境界線')}`,
    borderRadius: '4px',
})

// 進行名・和音要約・ボタンだけの1行目。利用中の札はここへ入れず2行目へ折り返す。
export const 独自進行の見出し行 = style({
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
    flexWrap: 'wrap',
})

// 編集・削除ボタンの列を1行目の右端へ固定し、進行名や和音要約が伸びても縮まない。
export const 独自進行のボタン列 = style([横並び行, {
    marginLeft: 'auto',
    flexShrink: 0,
}])

// いまその進行が使われているかを見せる札。見出し行から折り返した2行目へ全幅で置くため、
// サイドバーの幅より長い札名の並びでも折り返して収まるよう任意の位置で改行してよい。
export const 利用中の札 = style({
    display: 'block',
    fontSize: '10px',
    padding: '2px 6px',
    borderRadius: '3px',
    overflowWrap: 'anywhere',
    backgroundColor: エディターCSS変数('中立バッジ背景'),
    border: `1px solid ${エディターCSS変数('中立バッジ枠線')}`,
    color: エディターCSS変数('中立バッジ文字'),
    selectors: {
        '&[data-inuse="true"]': {
            backgroundColor: エディターCSS変数('アクセント背景'),
            borderColor: エディターCSS変数('アクセント背景'),
            color: エディターCSS変数('アクセント文字白'),
        },
    },
})

export const 編集枠 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
    padding: '12px',
    backgroundColor: エディターCSS変数('ビューポート背景'),
    border: `1px solid ${エディターCSS変数('境界線')}`,
    borderRadius: '4px',
})

export const 和音行枠 = style({
    display: 'flex',
    alignItems: 'center',
    gap: '6px',
    padding: '4px 6px',
    backgroundColor: エディターCSS変数('カード背景'),
    border: `1px solid ${エディターCSS変数('カード枠線')}`,
    borderRadius: '4px',
    flexWrap: 'wrap',
})

export const 和音番号 = style({
    fontSize: '11px',
    color: エディターCSS変数('テキスト薄'),
    minWidth: '20px',
})

export const 和音の欄 = style({
    width: 'auto',
    minWidth: '90px',
})
