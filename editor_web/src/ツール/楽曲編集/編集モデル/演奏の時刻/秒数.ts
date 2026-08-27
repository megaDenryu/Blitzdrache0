// 音声の時計が扱う時間の長さと時刻を表す値オブジェクト。
// 音声機器の時計は負の時刻を持たないため、0以上の有限な数値だけを受け付ける。
export class 秒数 {
    private constructor(private readonly _値: number) {}

    public static 生成する(値: number): 秒数 {
        if (!Number.isFinite(値) || 値 < 0) {
            throw new Error(`秒数は0以上の有限な数値でなければなりません: ${値}`)
        }
        return new 秒数(値)
    }

    public static ゼロ(): 秒数 {
        return new 秒数(0)
    }

    public 足す(他: 秒数): 秒数 {
        return 秒数.生成する(this._値 + 他._値)
    }

    // 引かれる側が引く側より前のときは、時間の向きが逆転しているため明示の失敗にする。
    public 差を求める(他: 秒数): 秒数 {
        return 秒数.生成する(this._値 - 他._値)
    }

    public 倍にする(倍率: number): 秒数 {
        if (!Number.isFinite(倍率) || 倍率 < 0) {
            throw new Error(`秒数の倍率は0以上の有限な数値でなければなりません: ${倍率}`)
        }
        return 秒数.生成する(this._値 * 倍率)
    }

    public 何回分か(間隔: 秒数): number {
        if (間隔._値 <= 0) {
            throw new Error('長さが0の間隔では何回分かを数えられません')
        }
        return this._値 / 間隔._値
    }

    public より前か(他: 秒数): boolean {
        return this._値 < 他._値
    }

    public 数値(): number {
        return this._値
    }
}
