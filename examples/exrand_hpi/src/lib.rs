#![feature(c_unwind)]

use log::trace;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rhsp3_common::*;
use rhsp3_plugsdk::{hsp_export, Var};
use std::time::SystemTime;

struct RngData(ChaCha8Rng);
impl HspExtData for RngData {
    fn init() -> Result<Self> {
        let time_diff = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        let key_a = time_diff.as_micros() as u64;
        let key_b = (time_diff.as_micros() >> 64) as u64;
        let key_c = std::process::id() as u64;
        let seed = (key_a ^ key_b ^ key_c).wrapping_mul(3202034522624059733);
        trace!("Reseeding from time. (Seed: {seed})");
        Ok(RngData(ChaCha8Rng::seed_from_u64(seed)))
    }
}

#[hsp_export]
fn ex_randomize(#[ext_data] rng: &mut RngData, seed: i32) -> Result<()> {
    trace!("Reseeding from seed {seed}.");
    *rng = RngData(ChaCha8Rng::seed_from_u64(seed as u64));
    Ok(())
}

#[hsp_export]
fn ex_randomize_time(#[ext_data] rng: &mut RngData) -> Result<()> {
    *rng = RngData::init()?;
    Ok(())
}

#[hsp_export]
fn ex_rand(#[ext_data] rng: &mut RngData, out: &mut impl Var<i32>, max: i32) -> Result<()> {
    ensure_code!(max >= 0, IllegalParameter);
    let val = rng.0.gen_range(0..max);
    out.set(val)?;
    Ok(())
}

#[hsp_export]
fn ex_rand2(
    #[ext_data] rng: &mut RngData,
    out: &mut impl Var<i32>,
    min: i32,
    range: i32,
) -> Result<()> {
    ensure_code!(range >= 0, IllegalParameter);
    let val = rng.0.gen_range(min..min + range);
    out.set(val)?;
    Ok(())
}

rhsp3_plugsdk::hpi_root!(pub ExrandHpi);
