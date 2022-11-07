#![feature(c_unwind)]

use log::trace;
use rand::{
    rngs::{adapter::ReseedingRng, OsRng},
    Rng, SeedableRng,
};
use rand_chacha::{ChaCha8Core, ChaCha8Rng};
use rhsp3_common::*;
use rhsp3_plugsdk::{hsp_export, Var};
use std::ops::Range;

fn new_seeded_rng() -> RngData {
    trace!("Reseeding from OS rng.");
    RngData::ReseedingRng(ReseedingRng::new(
        ChaCha8Core::from_rng(OsRng).expect("Could not retrieve randomness from `OsRng`."),
        16 * 1024,
        OsRng,
    ))
}

enum RngData {
    ReseedingRng(ReseedingRng<ChaCha8Core, OsRng>),
    FixedRng(ChaCha8Rng),
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
fn ex_randomize(#[ext_data] rng: &mut RngData, seed: i32) -> Result<()> {
    trace!("Reseeding from seed {seed}.");
    *rng = RngData::FixedRng(ChaCha8Rng::seed_from_u64(seed as u64));
    Ok(())
}

#[hsp_export]
fn ex_randomize_time(#[ext_data] rng: &mut RngData) -> Result<()> {
    *rng = new_seeded_rng();
    Ok(())
}

#[hsp_export]
fn ex_rand(#[ext_data] rng: &mut RngData, out: &mut impl Var<i32>, max: i32) -> Result<()> {
    ensure_code!(max >= 0, IllegalParameter);
    let val = rng.gen_range(0..max);
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
    let val = rng.gen_range(min..min + range);
    out.set(val)?;
    Ok(())
}

rhsp3_plugsdk::hpi_root!(ExrandHpi);
