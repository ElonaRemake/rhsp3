
			------�@MT�ɂ�闐������ EXrand v1.00�@------

�@HSP��rnd���߂͍ő�ł��͈�[0,32767]�̗��������o�͂ł��܂���B������ł����͈̗͂������o�����Ƃ����rnd���Q�Ƃ��Ĉ���Ƀr�b�g�V�t�g���܂��Ę_���a���Ċ|���Ċ����āc�ƐF�X�q�l��K�v�������āA�ʓ|�ł��B�����Ŋ�����rnd�ɗ���Ȃ����@���v���O�C���ɂ܂Ƃ߂Ă݂܂����B

					-*-*-

�@���̃v���O�C���ł́A���{�����A������m���ɂ��[�����������v���O���� Mersenne Twister(MT)���g���Ă��܂��B���{����MT��Web�y�[�W(http://www.math.keio.ac.jp/~matumoto/mt.html)�ɂ��΁AMT�ɂ͎��̂悤�ȓ���������܂��B
------
�E�]���ɂȂ�������, �������ϓ����z�������܂��B�i������2^19937-1�ŁA623�����������̂̒��� �ϓ��ɕ��z���邱�Ƃ��ؖ�����Ă��܂��B�j 
�E�������x�����Ȃ葬���B�i�����n�ɂ����܂����A�p�C�v���C��������L���b�V���������̂���V�X�e���ł́AC�̕W�����C�u������rand()��荂���Ȃ��Ƃ�����܂��B�j 
�E�������������ǂ��B�i32�r�b�g�ȏ�̃}�V���p�ɐ݌v���ꂽmt19937.c�́A624���[�h�̃��[�L���O������������邾���ł��B1���[�h��32�r�b�g���Ƃ��܂��B 
------

�c�܁[HSP���Ƃ��ł���Ȑ��x�̍��������g���ĉ�����񂶂�A�Ƃ͎v����ł�����(��
�@�Ȃ��MT�Ȃ̂����Ă����Ǝ������O����MT�̑��݂�m���Ă�������B������rnd�ł͂�����Ɣ͈͂�����Ȃ��Ȃ��āA�����@�����v���O�C���J���̕׋������˂�MT��HSP�v���O�C���ɂ��Ă݂悤�A���Ă����̂����@�ł��B

					-*-*-

�@�v���O�C���̗p�@�ł����A������exrand.as��common�ɁAexrand.dll��HSP�̂���t�H���_�ɒu���܂��B��ŁA���Ƃ̓R�[�h�̐擪������Ɏ��̈�s�����������܂��B

#include "exrand.as"

����ŏ��������Bex_randomize,ex_randomize_time,ex_rand,ex_rand2 �̂S���߂��L���ɂȂ�܂��B�e���߂̋@�\�Ɨp�@�����Ɏ����܂��B

��ex_randomize p1
�����n��̏������Bp1�͌n��ԍ��B���̒l��^���܂��傤�B
���̌n��ԍ���^���Ă�����Ζ��񓯂��������o�����܂��B

��ex_randomize_time
�����n������݂̎��Ԃ����Ƃɏ������B�Ƃ肠������������s���Ă����΁A����Ⴄ�����n��ɂȂ�܂��B

��ex_rand p1,p2
�ϐ�p1��0����p2-1�܂ł͈̔͂ŗ������o�́B
p2�ɂ�0�`$7fffffff�܂ł̒l���w�肵�Ă��������B
HSP��rnd�ƒu���������ł��܂��B

��ex_rand2 p1,p2,p3
�ϐ�p1��p2����p2+p3-1�܂ł͈̔͂ŗ������o�́B
p3�ɂ�0�`$7fffffff�܂ł̒l���w�肵�Ă��������B

					-*-*-

�@���s���x�Ƃ��͂܂��������Ă��Ă���܂���B���Ȃ��Ƃ�32768�ȏ�̒l���~�����ꍇ�ɂ́A�W����rnd���߂�HSP�R�[�h�𕹗p������͍����Ȃ͂��ł��B

�@�T���v���X�N���v�g sample.as��Y���Ă��܂��B�{���͕W��rnd�łЂ˂�o���������Ƃ̔�r���������ł������A�uHSP��rnd���߂�p����0�`100���͈̔͂ł����Ƃ΂���̂��闐�����o���v�A���S���Y�����l����̂��߂�ǂ������Ȃ��Ă�߂܂���(����

					-*-*-

�@�����Ƃ����̕ӂł����A����MT��BSD���C�Z���X�̉��ŔЕz����Ă��܂��B�ȉ���MT�̃\�[�X�t�@�C�����甲���������쌠�\���ƃ��C�Z���X�����ڂ��܂��B

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

�@�ŁA���̃v���O�C���ł����ǁA������t���[�\�t�g�E�F�A�����ɂ��܂��B���R�Ɏg���Ă��������B�ǂ����̒N������݂����ɏ��p�֎~�Ƃ��V�F�A�E�F�A�֎~�Ƃ��P�`�L�����Ƃ͌����܂���B�p�r�s��B�Ĕz�z�����R�B�����������l�����Ƀ\�[�X�����Ƃ��܂��BVC++6.0�p�B�ʓrHSPDLL.H������܂��Bonionsoft��HSP�y�[�W����E���Ă��Ă��������B

�@���񑩂ł����A���̒��̂��ׂĂ�Windows�}�V���œ��삷��ۏ؂͂���܂���B�܂����̃v���O�C���ɋN������G���[��OS���~�܂����Ƃ��A���̂�����������ăf�[�^���f�B�X�N���d���Ƃ����̑��������̃g���u�����N�����Ă��ӔC�͎��܂���B

--------------------
D.N.A.Softwares http://dna-softwares.go.to/
