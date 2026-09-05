import { globalStyle, style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../境界/index.ts'

export * from './スタイル/進行の帯.css.ts'
export * from './スタイル/トラック.css.ts'
export * from './スタイル/格子.css.ts'
export * from './スタイル/楽曲名の欄.css.ts'

// 中央は固定の2行(楽曲名と演奏の操作・和音の帯)とトラックの並びに分かれる。
// 縦に伸びるのはトラックの並びだけで、この枠自体はスクロールしない(設計正本の判断14)。
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

// 楽曲名の欄と演奏の操作帯を同じ行へ並べ、名前を出すためだけの行を作らない。
export const 固定の行 = style({
    display: 'flex',
    alignItems: 'stretch',
    gap: '12px',
    flexWrap: 'wrap',
    flexShrink: 0,
})

// 行の末尾に置く演奏の操作帯へ残り幅を渡す。楽曲名の欄は自分の幅で左端に留まる。
globalStyle(`${固定の行} > *:last-child`, {
    flex: '1',
    minWidth: '460px',
})

// タイムラインの行。楽曲名・操作帯の行の直下、和音の帯の行の上に置く固定の行で、
// 縦には伸びず、タイムライン部品の枠の中だけが横にスクロールする(設計正本の判断15)。
export const タイムラインの行 = style({
    display: 'flex',
    flexShrink: 0,
    minWidth: 0,
})

// flexの子はデフォルトでmin-width:autoのため、内容が幅を超えても縮まずoverflow-xが働かない。
// タイムライン部品へ横幅いっぱいを割り当て、縮んでよいことを明示する。
globalStyle(`${タイムラインの行} > *`, {
    flex: '1',
    minWidth: 0,
})

// 打ち込みの手が変わる進行制約の札を、和音の帯と同じ行の左端に置く。
export const 進行の行 = style({
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
    flexShrink: 0,
})

globalStyle(`${進行の行} > *:last-child`, {
    flex: '1',
    minWidth: '640px',
})

export const 情報バッジ = style({
    display: 'inline-flex',
    alignItems: 'center',
    gap: '4px',
    padding: '3px 8px',
    fontSize: '12px',
    borderRadius: '4px',
    whiteSpace: 'nowrap',
    flexShrink: 0,
    backgroundColor: エディターCSS変数('カード背景'),
    border: `1px solid ${エディターCSS変数('カード枠線')}`,
    color: エディターCSS変数('テキスト副'),
})

// 数が増えうるのはトラックだけであり、縦にスクロールするのはこの枠の中だけである。
export const トラックの並びの枠 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '12px',
    flex: '1',
    minHeight: 0,
    overflowY: 'auto',
    overflowX: 'auto',
    paddingBottom: '8px',
})
