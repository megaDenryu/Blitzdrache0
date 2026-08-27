import { style } from '@vanilla-extract/css'

// パターンパネルに固有の配置。外枠・見出し・入力欄・ボタンの見た目は共通のスタイルが持つ。
export const パターン操作行 = style({
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
    flexWrap: 'wrap',
})

export const 編集グリッド = style({
    display: 'grid',
    gridTemplateColumns: 'repeat(auto-fit, minmax(240px, 1fr))',
    gap: '12px',
})
