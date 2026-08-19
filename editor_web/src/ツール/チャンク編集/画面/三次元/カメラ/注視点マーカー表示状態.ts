// 注視点マーカーの可視状態を保持する純粋な状態機械。実時間の管理(タイマーの起動・取消)は
// 持たず、操作の通知と非表示化の指示だけを受けて可視/非可視を切り替える(副作用は
// 注視点マーカー表示制御器が持つ)。DOM・Three.jsに依存しないため単体テストできる。
export class 注視点マーカー表示状態 {
    private _可視か: boolean = false

    public 操作された(): void {
        this._可視か = true
    }

    public 非表示にする(): void {
        this._可視か = false
    }

    public 可視か(): boolean {
        return this._可視か
    }
}
