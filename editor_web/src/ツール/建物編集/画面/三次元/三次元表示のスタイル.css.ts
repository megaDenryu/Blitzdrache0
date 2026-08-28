import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../境界/index.ts'

// 三次元表示の枠。建物の形が編集の主役であるため、エディタ領域の中で与えられた箱を丸ごと埋める
// (高さを綴りで固定しない)。参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断14」
export const 三次元の枠 = style({
    position: 'relative',
    width: '100%',
    height: '100%',
    minWidth: 0,
    minHeight: 0,
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

// 視点の操作の案内。形の上へ薄く重ねるのは、縦の幅を案内のために取ると建物の見える範囲が
// そのぶん狭まるためである(設計正本の判断14)。触りを吸わないようポインタを通す。
export const 操作の案内 = style({
    position: 'absolute',
    left: '8px',
    bottom: '6px',
    fontSize: '11px',
    pointerEvents: 'none',
    color: エディターCSS変数('テキスト副'),
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
