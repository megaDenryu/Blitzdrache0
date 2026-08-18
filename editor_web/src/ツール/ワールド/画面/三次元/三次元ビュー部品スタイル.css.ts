import { style } from '@vanilla-extract/css'

export const コンテナ = style({
    position: 'absolute',
    inset: 0,
    width: '100%',
    height: '100%',
    overflow: 'hidden',
    backgroundColor: '#0b0f19',
})

export const キャンバス = style({
    display: 'block',
    width: '100%',
    height: '100%',
    outline: 'none',
})
