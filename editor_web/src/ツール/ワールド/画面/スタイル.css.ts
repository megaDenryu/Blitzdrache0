import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../境界/index.ts'

export const 画面ルート = style({
    position: 'relative',
    width: '100%',
    height: '100%',
    overflow: 'hidden',
    userSelect: 'none',
    backgroundColor: エディターCSS変数('ビューポート背景'),
})
