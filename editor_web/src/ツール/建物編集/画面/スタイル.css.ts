import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../境界/index.ts'

export * from './スタイル/平面図.css.ts'
export * from './スタイル/選択の並び.css.ts'
export * from './スタイル/建物名の欄.css.ts'
export * from './スタイル/インスペクター.css.ts'
export * from './スタイル/部品の棚.css.ts'

// エディタ領域は、上の固定の行と、その下の編集面に分かれる。編集面の中で縦に伸びるのは
// 三次元の枠と平面図だけで、この枠自体はスクロールしない(設計正本の判断14)。
export const コンテナ = style({
    width: '100%',
    height: '100%',
    boxSizing: 'border-box',
    padding: '12px 16px',
    backgroundColor: エディターCSS変数('ビューポート背景'),
    color: エディターCSS変数('テキスト主'),
    display: 'flex',
    flexDirection: 'column',
    gap: '12px',
    overflow: 'hidden',
})

// 建物の名前と、建物ぜんたいに効く操作を同じ行へ並べ、名前を出すためだけの行を作らない。
export const 固定の行 = style({
    display: 'flex',
    alignItems: 'center',
    gap: '12px',
    flexWrap: 'wrap',
    flexShrink: 0,
})

// 建物の形(三次元)を主に置き、その右へ触って編む平面図を並べる。どちらも人が編集中ずっと見ている。
export const 編集面 = style({
    display: 'flex',
    flexDirection: 'row',
    gap: '12px',
    flex: '1',
    minHeight: 0,
})

// 建物の形が編集の主役であるため、編集面の残り幅をすべてこの柱が受け取る。
export const 三次元の柱 = style({
    flex: '1',
    minWidth: 0,
    minHeight: 0,
    display: 'flex',
})

export const 平面図の柱 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '6px',
    width: '360px',
    flexShrink: 0,
    minHeight: 0,
})

// 升目は編むほど増えるため、平面図はこの枠の中だけでスクロールする。
export const 平面図の巻き取り枠 = style({
    flex: '1',
    minHeight: 0,
    overflow: 'auto',
})

// 筆が断られた事情を平面図の下へ出す帯。断りが無いときも高さを保ち、平面図の下が跳ねないようにする。
export const 触りの知らせ = style({
    minHeight: '16px',
    fontSize: '12px',
    flexShrink: 0,
})

export const 断りの文言 = style({
    color: エディターCSS変数('危険ボタン文字'),
})
