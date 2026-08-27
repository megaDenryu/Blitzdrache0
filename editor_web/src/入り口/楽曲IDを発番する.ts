import { 楽曲IDを生成する, type 楽曲ID } from '../境界/index.ts'

const 接頭辞 = 'song-'

// 新しい楽曲へ与える楽曲IDを1つ発番する純粋関数。
export function 楽曲IDを発番する(既にある識別子一覧: readonly 楽曲ID[]): 楽曲ID {
    const 使われている識別子 = new Set(既にある識別子一覧)
    for (let 番号 = 1; 番号 <= 既にある識別子一覧.length + 1; 番号 += 1) {
        const 候補 = 楽曲IDを生成する(`${接頭辞}${番号}`)
        if (!使われている識別子.has(候補)) return 候補
    }
    return 楽曲IDを生成する(`${接頭辞}${既にある識別子一覧.length + 1}`)
}

export function 新しい楽曲の既定の表示名(): string {
    return '新しい楽曲'
}
