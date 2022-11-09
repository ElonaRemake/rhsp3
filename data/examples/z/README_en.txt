z.hpi reimplementation
===========================

This directory contains a reimplementation of the z.hpi library in Rust, using
the rhsp3 library.

This is probably safer than using the original z.hpi, as it uses a fully memory
safe implementation of the gzip compression format instead of the original zlib
library.

Usage
-----

The original readme can be found in `z_jp.txt`, and a version translated into
English may be found in `z_en.txt`. They document the API that z.hpi uses.

While the API (in HSP code) is identical to that of the original library, it
is not compatible with .ax files compiled using the original. You must
recompile your program using the z.as provided with rhsp3.

Sample programs may be found in test_file.hsp and test_mem.hsp. They may be
used to test this library.

Links
-----

The source code for this reimplementation may be found here:
https://github.com/ElonaRemake/rhsp3/tree/main/examples/z_hpi

The original library (by MIA) may be found at this location:
http://taillove.jp/mia/plugin/arc/z.lzh

The original author's website may be found at this link:
http://taillove.jp/mia/

License
-------

This reimplementation is available under the public domain, but many libraries
it uses are not.

rhsp3 is available under the MIT license. For further information on the
licenses of other libraries rhsp3 uses, see THIRD_PARTY.txt in the main
directory.
