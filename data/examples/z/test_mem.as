#include "z.as"

#define DATASIZE	16384

	dim rdata, DATASIZE
	dim cdata, DATASIZE
	dim udata, DATASIZE

	repeat DATASIZE : rnd rdata.cnt : loop

	mes "���k���܂�"
	zSetDest  cdata, DATASIZE*4
	zCompress rdata, DATASIZE*4, 3
	writesize = stat
	rate = writesize * 100 / (DATASIZE * 4)

	mes "�o�̓T�C�Y "+writesize+" �o�C�g�i���k�� "+rate+"%�j"

	mes "\n�𓀂��܂�" : erf=0
	zSetDest    udata, DATASIZE*4
	zUncompress cdata, DATASIZE*4

	mes "���̃f�[�^�Ɣ�r��..." : erf=0
	repeat DATASIZE
		if udata.cnt!rdata.cnt : {
			mes ""+cnt+" : "+udata.cnt+" ! "+rdata.cnt
			erf=1
		}
	loop
	if erf=0 : mes "����_�͂���܂���"
stop