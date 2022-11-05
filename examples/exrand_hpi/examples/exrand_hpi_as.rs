pub fn main() {
    let header = rhsp3_plugsdk::codegen::make_hpi_headers::<exrand_hpi::ExrandHpi>("exrand.hpi");
    print!("{header}");
}
