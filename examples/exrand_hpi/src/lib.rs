#![feature(c_unwind)]

use rand::{
    rngs::{adapter::ReseedingRng, OsRng},
    Rng, SeedableRng,
};
use rand_chacha::{ChaCha12Core, ChaCha12Rng};
use rhsp3_common::*;
use rhsp3_plugsdk::{hsp_export, Var};
use std::ops::Range;

fn new_seeded_rng() -> RngData {
    RngData::ReseedingRng(ReseedingRng::new(
        ChaCha12Core::from_rng(OsRng).expect("Could not retrieve randomness from `OsRng`."),
        16 * 1024,
        OsRng,
    ))
}

enum RngData {
    ReseedingRng(ReseedingRng<ChaCha12Core, OsRng>),
    FixedRng(ChaCha12Rng),
}
impl HspExtData for RngData {
    fn init() -> Result<Self> {
        Ok(new_seeded_rng())
    }
}
impl RngData {
    fn gen_range(&mut self, range: Range<i32>) -> i32 {
        match self {
            RngData::ReseedingRng(rng) => rng.gen_range(range),
            RngData::FixedRng(rng) => rng.gen_range(range),
        }
    }
}

#[hsp_export]
fn exrand_randomize(#[ext_data] rng: &mut RngData, seed: i32) -> Result<()> {
    *rng = RngData::FixedRng(ChaCha12Rng::seed_from_u64(seed as u64));
    Ok(())
}

#[hsp_export]
fn exrand_randomize_by_time(#[ext_data] rng: &mut RngData) -> Result<()> {
    *rng = new_seeded_rng();
    Ok(())
}

#[hsp_export]
fn exrand_rnd(#[ext_data] rng: &mut RngData, out: &mut impl Var<i32>, max: i32) -> Result<()> {
    ensure_code!(max >= 0, IllegalParameter);
    let val = rng.gen_range(0..max);
    out.set(val)?;
    Ok(())
}

#[hsp_export]
fn exrand_rnd2(
    #[ext_data] rng: &mut RngData,
    out: &mut impl Var<i32>,
    min: i32,
    range: i32,
) -> Result<()> {
    ensure_code!(range >= 0, IllegalParameter);
    let val = rng.gen_range(min..min + range);
    out.set(val)?;
    Ok(())
}
