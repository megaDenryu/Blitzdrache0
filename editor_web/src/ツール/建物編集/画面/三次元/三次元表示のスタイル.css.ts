import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../境界/index.ts'

// 三次元の識別色表示の枠。平面図と並べて出すため、高さを固定した箱の中へキャンバスを敷く。
export const 三次元の枠 = style({
    position: 'relative',
    width: '100%',
    height: '320px',
    overflow: 'hidden',
    borderRadius: '4px',
    backgroundColor: エディターCSS変数('ビューポート背景'),
})

export const 三次元のキャンバス = style({
    display: 'block',
    width: '100%',
    height: '100%',
    outline: 'none',
})

export const 凡例の並び = style({
    display: 'flex',
    flexWrap: 'wrap',
    gap: '12px',
    fontSize: '12px',
})

export const 凡例の1件 = style({
    display: 'flex',
    alignItems: 'center',
    gap: '6px',
})

export const 凡例の色見本 = style({
    width: '12px',
    height: '12px',
    borderRadius: '2px',
})
