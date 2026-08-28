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
    // 右サイドバーの幅は利用者が変えられるため、列の最小幅を枠の幅で頭打ちにする。
    // min()を挟まないと、枠より広い最小幅が横のはみ出しになる。
    gridTemplateColumns: 'repeat(auto-fit, minmax(min(240px, 100%), 1fr))',
    gap: '12px',
})
