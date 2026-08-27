import type { 音の並び } from '../../../生成/編集資源契約.ts'

const 音名一覧: readonly string[] = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B']

// 音高番号から音名表記（例: C4, A#3）を組み立てる。
export function 音高番号の音名表記(音高番号: number): string {
    const ピッチクラス = ((音高番号 % 12) + 12) % 12
    const オクターブ = Math.floor(音高番号 / 12) - 1
    const 音名 = 音名一覧[ピッチクラス]
    return 音名 === undefined ? `Note-${音高番号}` : `${音名}${オクターブ}`
}

// トラックの行位置に対応する行の表示名を組み立てる。
export function トラック行の表示名(音の並び: 音の並び, 行位置: number): string {
    if (音の並び.種類 === '音高の行一覧') {
        const 音高番号 = 音の並び.値[行位置]
        return 音高番号 === undefined ? `行${行位置}` : 音高番号の音名表記(音高番号)
    }
    const 打楽器 = 音の並び.値[行位置]
    return 打楽器 === undefined ? `行${行位置}` : 打楽器
}
