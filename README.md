# rust-interpreter

Rustで実装しているMonkey言語のインタプリタです。

参考：「Writing An Interpreter In Go」

## 現在できること

- 整数・真偽値・文字列リテラル
- 四則演算・比較演算
- `let` による変数束縛
- `if` / `else`
- `return`
- 関数定義・関数呼び出し
- クロージャ
- 組み込み関数 `len`, `puts`
- REPLでの実行

## 使い方

REPLを起動します。

```bash
cargo run
```

例:

```monkey
> let add = fn(a, b) { a + b }
> add(2, 3)
5
> len("hello")
5
> puts("Hello, Monkey!")
Hello, Monkey!
```

終了するときは `exit` または `quit` を入力します。

```monkey
> exit
```

## 動作確認

テストを実行します。

```bash
cargo test
```

型チェックだけを素早く行う場合:

```bash
cargo check
```

## 今後やりたいこと

- エラーオブジェクトの追加
- 配列
- ハッシュ
- 組み込み関数の追加
