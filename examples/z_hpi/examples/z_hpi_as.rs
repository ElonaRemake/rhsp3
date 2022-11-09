pub fn main() {
    let header = rhsp3_plugsdk::codegen::make_hpi_headers::<z_hpi::ZlibHpi>("z.hpi");
    print!("{header}");
}
