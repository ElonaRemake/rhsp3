日本語があまり下手だから、日本語のドキュメンテーションが少ないです。ごめんなさい。
元のドキュメンテーションはexrand_jp.txtにあります。

元のexrand.asでコンパイルされた.axファイルはrhsp3のexrand.hpiと共に使用できません。
そのため、rhsp3のexrand.asを使ってコンパイルすることが必要です。

この実装はMTよりにChaCha8乱数生成を使います。
そのため、ex_randomizeを呼び出しても乱数列は元から違います。

ソースコードは https://github.com/ElonaRemake/rhsp3/tree/main/examples/exrand_hpi で公開します。
英語版だけがありますでごめんなさい。

元のexrandのURLは消したでもweb.archive.orgで保存したバージョンがまだあります。
このURLでダウンロードされます:
http://web.archive.org/web/20051212151105/http://www.infobears.ne.jp/athome/mn_ezaki/hsp/exrand_100.lzh
そのプラグインの作者のウエブページはここであります: https://www.dna-softwares.com/

このプラグインはrhsp3_plugapiの使い方を教えるためだけで書きました。
本当に、乱数生成のプラグインが必要なら、元のexrandを使うほうがいいと思う… ^^;
その上…HSP3のrndの範囲はHSP2と同じ限界がないだから、rndを使っても問題がないよね…
