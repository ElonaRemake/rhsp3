exrand.hpi reimplementation
===========================

This directory contains a reimplementation of the exrand.hpi library in Rust,
using the rhsp3 library.

This is meant more as an example of how to write plugins using rhsp3_plugapi,
then a new version of exrand.hpi. You are probably better off using the
original if you need a random number plugin for HSP.

Or... umm... just the rnd function. As of HSP3, it no longer has range limits.

Usage
-----

The original readme can be found in `exrand_jp.txt`, and a version translated
into English may be found in `exrand_en.txt`. They document the API that exrand
uses.

While the API (in HSP code) is identical to that of exrand, it is not
compatible with .ax files compiled using the original. You must recompile your
program using the exrand.as provided with rhsp3.

Furthermore, this implementation uses the ChaCha8 algorithm rather than the
Mersenne Twister algorithm. Therefore, the output will be different from the
original even when the ex_randomize function is called.

A sample program be found at sample_en.hsp. It can be used to test this library.

Links
-----

The source code for this reimplementation may be found here:
https://github.com/ElonaRemake/rhsp3/tree/main/examples/exrand_hpi

While the original version of exrand has since disappeared off the face of the
internet, a copy can still be found archived on the Internet Library:
http://web.archive.org/web/20051212151105/http://www.infobears.ne.jp/athome/mn_ezaki/hsp/exrand_100.lzh

The original author's website may still be found at the following link. exrand
(or any mention of HSP) can no longer be found on it.
https://www.dna-softwares.com/

License
-------

This reimplementation is available under the public domain, but many libraries
it uses are not.

rhsp3 is available under the MIT license. For further information on the
licenses of other libraries rhsp3 uses, see THIRD_PARTY.txt in the main
directory.
