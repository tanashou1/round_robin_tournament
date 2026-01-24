# 総当たりトーナメント作成ツール

総当たりトーナメント（ラウンドロビン方式）の試合スケジュールを生成するツールです。

## オンライン版

GitHub Pagesで公開されています：
https://tanashou1.github.io/round_robin_tournament/

## 機能

- チーム名を改行区切りで入力
- コート数を指定
- WebAssemblyで高速な総当たり戦の組合せ生成
- 結果をCSV形式でダウンロード可能

## ローカル開発

### 必要なもの

- Rust (最新版)
- Node.js 20以上
- wasm-pack

### セットアップ

1. リポジトリをクローン
```bash
git clone https://github.com/tanashou1/round_robin_tournament.git
cd round_robin_tournament
```

2. WebAssemblyモジュールをビルド
```bash
wasm-pack build --target web --out-dir pkg
```

3. Reactアプリケーションのセットアップ
```bash
cd web
npm install
npm run dev
```

4. ブラウザで http://localhost:5173/round_robin_tournament/ を開く

### プロダクションビルド

```bash
# WebAssemblyをビルド
wasm-pack build --target web --out-dir pkg

# Reactアプリをビルド
cd web
npm run build
```

## CLIツール

WebUIのほかに、コマンドラインツールも利用できます：

```bash
cargo run --features cli
```

チーム数またはExcelファイルのパスを入力し、コート数を指定すると、`組合せ結果.csv` ファイルが生成されます。

## 技術スタック

- **Backend**: Rust + WebAssembly (wasm-bindgen)
- **Frontend**: React + Vite
- **Deployment**: GitHub Pages

## ライセンス

MIT
