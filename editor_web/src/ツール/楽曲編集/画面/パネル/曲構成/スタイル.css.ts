import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

// 曲構成パネルに固有の配置。外枠・見出し・入力欄・ボタン・帯の見た目は共通のスタイルが持つ。
export const 節一覧コンテナ = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
})

export const 節行枠 = style({
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
    padding: '8px 12px',
    backgroundColor: エディターCSS変数('ビューポート背景'),
    border: `1px solid ${エディターCSS変数('境界線')}`,
    borderRadius: '4px',
    flexWrap: 'wrap',
})

export const 節番号 = style({
    fontSize: '11px',
    color: エディターCSS変数('テキスト薄'),
    minWidth: '24px',
})

export const 回数選択 = style({
    minWidth: '90px',
    width: 'auto',
})

export const 操作ボタン群 = style({
    display: 'flex',
    alignItems: 'center',
    gap: '4px',
})

export const 追加ボタン = style({
    alignSelf: 'flex-start',
})
