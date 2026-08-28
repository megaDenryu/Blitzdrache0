import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

// 三次元を持つエディタ領域の骨組み。上に固定の行を1本置き、その下の残りをすべて三次元の枠へ渡す。
// チャンク編集と大域編集が同じ骨組みを参照するのは、区画の役割が両方で同じであり、
// 別々に書くと片方だけがずれるためである(設計正本の判断14)。
// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断14」
export const 画面ルート = style({
    position: 'relative',
    width: '100%',
    height: '100%',
    overflow: 'hidden',
    userSelect: 'none',
    display: 'flex',
    flexDirection: 'column',
    backgroundColor: エディターCSS変数('ビューポート背景'),
})

// いま何を編集しているかと、対象の全体に効く操作を収める行。三次元と一緒にスクロールして
// 消えてはならないため、縮まない行として置く。
export const 固定の行 = style({
    flexShrink: 0,
    padding: '8px 12px',
    borderBottom: `1px solid ${エディターCSS変数('境界線')}`,
    backgroundColor: エディターCSS変数('パネル背景'),
})

// 三次元ビュー部品は自分の枠へ絶対配置で広がるため、位置の基準をこの枠が与える。
export const 三次元の枠 = style({
    position: 'relative',
    flex: '1',
    minHeight: 0,
})
