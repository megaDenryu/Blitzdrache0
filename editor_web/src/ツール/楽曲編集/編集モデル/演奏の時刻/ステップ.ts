// 16分音符1つ分の位置および長さを表す値オブジェクト。1拍は4ステップ、1小節は16ステップである。
export class ステップ {
    private constructor(private readonly _値: number) {}

    public static 生成する(値: number): ステップ {
        if (!Number.isInteger(値) || 値 < 0) {
            throw new Error(`ステップは0以上の整数でなければなりません: ${値}`)
        }
        return new ステップ(値)
    }

    public static ゼロ(): ステップ {
        return new ステップ(0)
    }

    public 足す(他: ステップ): ステップ {
        return ステップ.生成する(this._値 + 他._値)
    }

    public 次(): ステップ {
        return ステップ.生成する(this._値 + 1)
    }

    // 曲の長さで折り返した位置を返す。演奏は曲の終わりで先頭へ戻るため、剰余はこの型が持つ。
    public 全体の長さで折り返す(全体の長さ: ステップ): ステップ {
        if (全体の長さ._値 <= 0) {
            throw new Error('全体の長さが0のステップでは折り返せません')
        }
        return ステップ.生成する(this._値 % 全体の長さ._値)
    }

    public 同じか(他: ステップ): boolean {
        return this._値 === 他._値
    }

    public 数値(): number {
        return this._値
    }
}
