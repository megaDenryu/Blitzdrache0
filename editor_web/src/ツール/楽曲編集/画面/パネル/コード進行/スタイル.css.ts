import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

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

export const 独自進行の見出し行 = style({
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    gap: '8px',
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
