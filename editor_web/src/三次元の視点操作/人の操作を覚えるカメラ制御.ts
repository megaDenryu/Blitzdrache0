import type { 位置3次元 } from '../生成/編集資源契約.ts'
import type { 軌道カメラ制御器 } from './軌道カメラ制御器.ts'

// 軌道カメラ制御器を包み、人が一度でも視点を動かしたことを覚える制御。
// 編集の対象ぜんたいが写るようカメラを合わせ直す道具では、人が視点を決めた後にも合わせ直すと、
// 人が決めたことを機械が黙って上書きすることになる。合わせ直してよいかの判断材料をここが持つ。
// 参照: `~/.claude/skills/エディター制作`「第1条: 人が決めることを機械が黙って決めない」
export class 人の操作を覚えるカメラ制御 {
    private _人が動かしたか = false

    public constructor(private readonly _中身: 軌道カメラ制御器) {}

    public get 人が動かしたか(): boolean {
        return this._人が動かしたか
    }

    public 回転する(デルタX: number, デルタY: number): void {
        this._人が動かしたか = true
        this._中身.回転する(デルタX, デルタY)
    }

    public 移動する(デルタX: number, デルタY: number): void {
        this._人が動かしたか = true
        this._中身.移動する(デルタX, デルタY)
    }

    public 拡大縮小する(ホイールの移動量: number): void {
        this._人が動かしたか = true
        this._中身.拡大縮小する(ホイールの移動量)
    }

    public 注視点を取得する(): 位置3次元 {
        return this._中身.注視点を取得する()
    }
}
