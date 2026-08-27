import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

// トラック設定パネルに固有の配置。外枠・見出し・入力欄の見た目は共通のスタイルが持つ。
export const トラック行一覧 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '12px',
})

export const トラック設定行枠 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
    padding: '10px 12px',
    backgroundColor: エディターCSS変数('ビューポート背景'),
    border: `1px solid ${エディターCSS変数('境界線')}`,
    borderRadius: '4px',
})

export const トラック見出し行 = style({
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
})

export const トラック名 = style({
    fontSize: '12px',
    fontWeight: 600,
    color: エディターCSS変数('テキスト主'),
})

export const 種類バッジ = style({
    padding: '2px 6px',
    fontSize: '10px',
    borderRadius: '3px',
    backgroundColor: エディターCSS変数('カード背景'),
    border: `1px solid ${エディターCSS変数('カード枠線')}`,
    color: エディターCSS変数('テキスト副'),
})

export const トラック項目群 = style({
    display: 'grid',
    gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))',
    gap: '8px',
    alignItems: 'end',
})
