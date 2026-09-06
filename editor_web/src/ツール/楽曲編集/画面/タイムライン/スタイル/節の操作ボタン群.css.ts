import { style } from '@vanilla-extract/css'

// 節の枠見出しに置く、削除・複製・前後挿入・前後移動の6つのボタンを並べる行。
export const 操作ボタン行 = style({
    display: 'flex',
    flexWrap: 'wrap',
    gap: '2px',
    marginTop: '2px',
})
