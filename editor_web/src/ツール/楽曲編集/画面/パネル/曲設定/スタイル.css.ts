import { style } from '@vanilla-extract/css'

// 曲設定パネルに固有の配置。外枠・見出し・入力欄の見た目は共通のスタイルが持つ。
export const ミキサーグリッド = style({
    display: 'grid',
    gridTemplateColumns: 'repeat(auto-fit, minmax(220px, 1fr))',
    gap: '12px',
})
