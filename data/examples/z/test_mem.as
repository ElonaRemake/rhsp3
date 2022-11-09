#include "z.as"

#define DATASIZE	16384

	dim rdata, DATASIZE
	dim cdata, DATASIZE
	dim udata, DATASIZE

	repeat DATASIZE : rnd rdata.cnt : loop

	mes "圧縮します"
	zSetDest  cdata, DATASIZE*4
	zCompress rdata, DATASIZE*4, 3
	writesize = stat
	rate = writesize * 100 / (DATASIZE * 4)

	mes "出力サイズ "+writesize+" バイト（圧縮率 "+rate+"%）"

	mes "\n解凍します" : erf=0
	zSetDest    udata, DATASIZE*4
	zUncompress cdata, DATASIZE*4

	mes "元のデータと比較中..." : erf=0
	repeat DATASIZE
		if udata.cnt!rdata.cnt : {
			mes ""+cnt+" : "+udata.cnt+" ! "+rdata.cnt
			erf=1
		}
	loop
	if erf=0 : mes "相違点はありません"
stop