import { style } from '@vanilla-extract/css'

export const 行コンテナ = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '4px',
})

export const ラベル行 = style({
    display: 'flex',
    justifyContent: 'space-between',
    fontSize: '11px',
    color: '#94a3b8',
})

export const 値ラベル = style({
    fontFamily: 'monospace',
    color: '#22d3ee',
})

export const スライダー入力 = style({
    width: '100%',
    accentColor: '#06b6d4',
    cursor: 'pointer',
})
