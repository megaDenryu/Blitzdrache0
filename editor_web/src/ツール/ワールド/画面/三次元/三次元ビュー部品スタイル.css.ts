import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../境界/index.ts'

export const コンテナ = style({
    position: 'absolute',
    inset: 0,
    width: '100%',
    height: '100%',
    overflow: 'hidden',
    backgroundColor: エディターCSS変数('ビューポート背景'),
})

export const キャンバス = style({
    display: 'block',
    width: '100%',
    height: '100%',
    outline: 'none',
})
