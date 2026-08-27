import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../境界/index.ts'

// 三次元表示の枠。平面図と並べて出すため、高さを固定した箱の中へキャンバスを敷く。
// 回して形を確かめる表示であるため、平面図の下に置く箱としては大きめに取る。
export const 三次元の枠 = style({
    position: 'relative',
    width: '100%',
    height: '420px',
    overflow: 'hidden',
    borderRadius: '4px',
    backgroundColor: エディターCSS変数('ビューポート背景'),
})

export const 三次元のキャンバス = style({
    display: 'block',
    width: '100%',
    height: '100%',
    outline: 'none',
    // 右ドラッグで回し左クリックで選ぶため、掴める面であることを形で示す。
    cursor: 'grab',
})

// いま選んでいる升目を綴りでも出す帯。三次元の枠の中だけで示すと、枠の外を見ている人に伝わらない。
export const 選択の知らせ = style({
    minHeight: '16px',
    fontSize: '12px',
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
