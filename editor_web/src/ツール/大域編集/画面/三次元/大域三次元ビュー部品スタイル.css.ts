import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../境界/index.ts'

export const コンテナ = style({
    width: '100%',
    height: '100%',
    position: 'relative',
    overflow: 'hidden',
    backgroundColor: エディターCSS変数('ビューポート背景'),
})

export const キャンバス = style({
    width: '100%',
    height: '100%',
    display: 'block',
    outline: 'none',
})
