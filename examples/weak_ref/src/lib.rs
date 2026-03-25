//! Weak reference APIs example.

use std::thread;
use std::time::Duration;

use ani::prelude::*;
use ani_derive::ani;

const PRESSURE_OBJECTS_PER_ROUND: usize = 256;
const PRESSURE_STRINGS_PER_ROUND: usize = 32;
const PRESSURE_STRING_BYTES: usize = 512;

fn fresh_object<'env>(env: &Env<'env>) -> Result<AniObject<'env>> {
    let class = env.find_class("std.core.Object")?;
    let ctor = env.find_constructor(&class, ":")?;
    env.new_object(&class, &ctor, &[])
}

fn fresh_weak_ref(env: &Env<'_>) -> Result<WeakRef> {
    let _scope = env.create_local_scope(8)?;
    let object = fresh_object(env)?;
    let object_ref: AniRef<'_> = object.into();
    env.create_weak_ref(&object_ref)
}

fn fresh_weak_global_pair(env: &Env<'_>) -> Result<(WeakRef, GlobalRef)> {
    let _scope = env.create_local_scope(8)?;
    let object = fresh_object(env)?;
    let object_ref: AniRef<'_> = object.into();
    let weak = env.create_weak_ref(&object_ref)?;
    let global = env.create_global_ref(&object_ref)?;
    Ok((weak, global))
}

fn allocation_pressure_round(env: &Env<'_>, round: usize) -> Result<()> {
    let _scope =
        env.create_local_scope(PRESSURE_OBJECTS_PER_ROUND + PRESSURE_STRINGS_PER_ROUND + 8)?;
    let payload = "x".repeat(PRESSURE_STRING_BYTES);

    for _ in 0..PRESSURE_OBJECTS_PER_ROUND {
        let _ = fresh_object(env)?;
    }

    for idx in 0..PRESSURE_STRINGS_PER_ROUND {
        let _ = env.create_string(&format!("{round}:{idx}:{payload}"))?;
    }

    Ok(())
}

fn weak_is_alive(env: &Env<'_>, weak: &WeakRef) -> Result<bool> {
    weak.is_alive(env)
}

fn weak_survives_pressure(env: &Env<'_>, weak: &WeakRef, rounds: usize) -> Result<bool> {
    for round in 0..rounds.max(1) {
        if !weak_is_alive(env, weak)? {
            return Ok(false);
        }
        allocation_pressure_round(env, round)?;
    }

    weak_is_alive(env, weak)
}

fn weak_releases_under_pressure(env: &Env<'_>, weak: &WeakRef, rounds: usize) -> Result<bool> {
    if !weak_is_alive(env, weak)? {
        return Ok(true);
    }

    for round in 0..rounds.max(1) {
        allocation_pressure_round(env, round)?;
        if !weak_is_alive(env, weak)? {
            return Ok(true);
        }
        if round % 32 == 31 {
            thread::sleep(Duration::from_millis(1));
        }
    }

    Ok(false)
}

#[ani]
pub fn weak_ref_roundtrip(env: &Env<'_>, value: AniRef<'_>) -> Result<bool> {
    let weak = env.create_weak_ref(&value)?;
    let upgraded = weak.is_alive(env)?;
    weak.delete(env)?;
    Ok(upgraded)
}

#[ani]
pub fn weak_ref_releases_after_pressure(env: &Env<'_>, rounds: i32) -> Result<bool> {
    let weak = fresh_weak_ref(env)?;

    let released = weak_releases_under_pressure(env, &weak, rounds.max(1) as usize);
    let delete = weak.delete(env);

    match (released, delete) {
        (Ok(result), Ok(())) => Ok(result),
        (Err(err), _) => Err(err),
        (Ok(_), Err(err)) => Err(err),
    }
}

#[ani]
pub fn weak_ref_survives_global_ref_pressure(env: &Env<'_>, rounds: i32) -> Result<bool> {
    let rounds = rounds.max(1) as usize;
    let (weak, global) = fresh_weak_global_pair(env)?;
    let survived = weak_survives_pressure(env, &weak, rounds);
    let delete_global = global.delete(env);
    let delete_weak = weak.delete(env);

    match (survived, delete_global, delete_weak) {
        (Ok(result), Ok(()), Ok(())) => Ok(result),
        (Err(err), _, _) => Err(err),
        (_, Err(err), _) => Err(err),
        (_, _, Err(err)) => Err(err),
    }
}

#[ani]
pub fn weak_ref_releases_after_global_drop(env: &Env<'_>, rounds: i32) -> Result<bool> {
    let rounds = rounds.max(1) as usize;
    let (weak, global) = fresh_weak_global_pair(env)?;
    let started_alive = weak_is_alive(env, &weak);
    let delete_global = global.delete(env);
    let released_after_drop =
        weak_releases_under_pressure(env, &weak, rounds.saturating_mul(16));
    let delete_weak = weak.delete(env);

    match (started_alive, delete_global, released_after_drop, delete_weak) {
        (Ok(alive), Ok(()), Ok(released), Ok(())) => Ok(alive && released),
        (Err(err), _, _, _) => Err(err),
        (_, Err(err), _, _) => Err(err),
        (_, _, Err(err), _) => Err(err),
        (_, _, _, Err(err)) => Err(err),
    }
}

#[ani]
pub fn weak_ref_tracks_global_ref_lifecycle(env: &Env<'_>, rounds: i32) -> Result<bool> {
    Ok(
        weak_ref_survives_global_ref_pressure(env, rounds)?
            && weak_ref_releases_after_global_drop(env, rounds)?,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_signatures_compile() {
        let _ = weak_ref_roundtrip;
        let _ = weak_ref_releases_after_pressure;
        let _ = weak_ref_survives_global_ref_pressure;
        let _ = weak_ref_releases_after_global_drop;
        let _ = weak_ref_tracks_global_ref_lifecycle;
    }
}
