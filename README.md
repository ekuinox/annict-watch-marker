# annict-watch-marker

[ekuinox/animetick-exporter-bookmarklet](https://github.com/ekuinox/animetick-exporter-bookmarklet)で作ったJSONを読み込んで、annictに登録するやつ

## usage

1. `$ cargo install --git https://github.com/ekuinox/annict-watch-marker.git`
2. [annict](https://annict.com/settings/tokens/new)から個人用アクセストークンを発行 (R/W)
3. `$ ANNICT_TOKEN=<個人用アクセストークン> annict-watch-marker <ANIMETICK JSON DIRECTORY>`

## memo

- serdeで書いた型が適当過ぎてメチャエラー出る
- アニメのタイトルで検索してるから、記号とかが含まれているとAnnict上で見つけられない
- しょっちゅうToo Many Requestsが出る
