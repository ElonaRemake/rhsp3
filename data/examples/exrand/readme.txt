
			------　MTによる乱数生成 EXrand v1.00　------

　HSPのrnd命令は最大でも範囲[0,32767]の乱数しか出力できません。これよりでかい範囲の乱数を出そうとするとrndを２つとって一方にビットシフトかまして論理和して掛けて割って…と色々ヒネる必要があって、面倒です。そこで既存のrndに頼らない方法をプラグインにまとめてみました。

					-*-*-

　このプラグインでは、松本眞氏、西村拓士氏による擬似乱数生成プログラム Mersenne Twister(MT)を使っています。松本氏のMTのWebページ(http://www.math.keio.ac.jp/~matumoto/mt.html)によれば、MTには次のような特徴があります。
------
・従来にない長周期, 高次元均等分布を持ちます。（周期が2^19937-1で、623次元超立方体の中に 均等に分布することが証明されています。） 
・生成速度がかなり速い。（処理系にもよりますが、パイプライン処理やキャッシュメモリのあるシステムでは、Cの標準ライブラリのrand()より高速なこともあります。） 
・メモリ効率が良い。（32ビット以上のマシン用に設計されたmt19937.cは、624ワードのワーキングメモリを消費するだけです。1ワードは32ビット長とします。 
------

…まーHSPごときでそんな精度の高い乱数使って何するんじゃ、とは思うんですけど(ｗ
　なんでMTなのかっていうと自分が前からMTの存在を知っていたから。既存のrndではちょっと範囲が足りなくなって、いい機会だからプラグイン開発の勉強もかねてMTをHSPプラグインにしてみよう、っていうのが動機です。

					-*-*-

　プラグインの用法ですが、同梱のexrand.asをcommonに、exrand.dllをHSPのあるフォルダに置きます。んで、あとはコードの先頭あたりに次の一行を書き加えます。

#include "exrand.as"

これで準備完了。ex_randomize,ex_randomize_time,ex_rand,ex_rand2 の４命令が有効になります。各命令の機能と用法を次に示します。

○ex_randomize p1
乱数系列の初期化。p1は系列番号。正の値を与えましょう。
一定の系列番号を与えてあげれば毎回同じ乱数が出現します。

○ex_randomize_time
乱数系列を現在の時間をもとに初期化。とりあえずこれを実行しておけば、毎回違う乱数系列になります。

○ex_rand p1,p2
変数p1に0からp2-1までの範囲で乱数を出力。
p2には0～$7fffffffまでの値を指定してください。
HSPのrndと置き換えができます。

○ex_rand2 p1,p2,p3
変数p1にp2からp2+p3-1までの範囲で乱数を出力。
p3には0～$7fffffffまでの値を指定してください。

					-*-*-

　実行速度とかはまったく勘案しておりません。少なくとも32768以上の値が欲しい場合には、標準のrnd命令とHSPコードを併用するよりは高速なはずです。

　サンプルスクリプト sample.asを添えています。本当は標準rndでひねり出した乱数との比較をするつもりでしたが、「HSPのrnd命令を用いて0～100万の範囲でちゃんとばらつきのある乱数を出す」アルゴリズムを考えるのがめんどくさくなってやめました(ぉぃ

					-*-*-

　権利とかその辺ですが、元のMTはBSDライセンスの下で頒布されています。以下にMTのソースファイルから抜粋した著作権表示とライセンス文を載せます。

   Copyright (C) 1997 - 2002, Makoto Matsumoto and Takuji Nishimura,
   All rights reserved.

   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

     1. Redistributions of source code must retain the above copyright
        notice, this list of conditions and the following disclaimer.

     2. Redistributions in binary form must reproduce the above copyright
        notice, this list of conditions and the following disclaimer in the
        documentation and/or other materials provided with the distribution.

     3. The names of its contributors may not be used to endorse or promote 
        products derived from this software without specific prior written 
        permission.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE COPYRIGHT OWNER OR
   CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

　で、このプラグインですけど、いわゆるフリーソフトウェア扱いにします。自由に使ってください。どっかの誰かさんみたいに商用禁止とかシェアウェア禁止とかケチ臭いことは言いません。用途不問。再配布も自由。改造したい人向けにソースもつけときます。VC++6.0用。別途HSPDLL.Hがいります。onionsoftのHSPページから拾ってきてください。

　お約束ですが、世の中のすべてのWindowsマシンで動作する保証はありません。またこのプラグインに起因するエラーでOSが止まったとか、そのあおりを喰らってデータやらディスクが㌧だとかその他もろもろのトラブルが起こっても責任は取りません。

--------------------
D.N.A.Softwares http://dna-softwares.go.to/
