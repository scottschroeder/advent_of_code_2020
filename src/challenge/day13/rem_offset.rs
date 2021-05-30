use anyhow::Result;

pub fn chain_offset(c: &[Option<i64>]) -> Result<i64> {
    let init = match c.iter().next() {
        Some(Some(x)) => *x,
        _ => anyhow::bail!("chain must start with valid entry"),
    };

    struct State {
        start: i64,
        interval: i64,
    }

    let mut state = State {
        start: 0,
        interval: init,
    };

    for (idx, id) in c.iter().enumerate().skip(1).filter_map(|(idx, id)| {
        if let Some(id) = id {
            Some((idx, *id))
        } else {
            None
        }
    }) {
        let (offset, period) = offset_rem(state.interval, id, idx as i64, state.start)?;
        state.start = offset;
        state.interval = period;
    }

    Ok(state.start)
}

pub fn offset_rem(c1: i64, c2: i64, target: i64, c1_offset: i64) -> Result<(i64, i64)> {
    let period = c1 * c2;

    let im = aoc::math::inverse_mod(c1, c2).ok_or_else(|| {
        anyhow::anyhow!(
            "two numbers ({} & {}) do not have an inverse mod, so won't ever align",
            c1,
            c2
        )
    })?;

    let c2_offset = c1_offset % c2;
    let c2_target = c2 - target;
    let c2_steps = c2_target - c2_offset;
    let mut offset = (c2_steps * im * c1 + c1_offset) % period;

    while offset < 0 {
        offset += period
    }

    log::trace!(
        "c1: {} - c2: {} - im: {} - target: {} - c1_off: {} - period: {}",
        c1,
        c2,
        im,
        target,
        c1_offset,
        period
    );
    log::trace!(
        "c2_offset: {} c2_target: {} c2_steps: {} final: {}",
        c2_offset,
        c2_target,
        c2_steps,
        offset
    );

    log::debug!("offset: {}, period: {}", offset, c1 * c2);
    Ok((offset, period))
}

pub fn print_grid(ids: &[&i64], start: usize, end: usize) {
    print!("time\t");
    for id in ids {
        print!("{}\t", id);
    }
    println!("");
    for idx in start..end {
        print!("{}\t", idx);

        for id in ids {
            if (idx as i64) % *id == 0 {
                print!("D\t")
            } else {
                print!(".\t")
            }
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_offset_consecutive_small_large() {
        assert_eq!(offset_rem(3, 5, 1, 0).unwrap(), (9, 15));
    }

    #[test]
    fn offset_consecutive() {
        assert_eq!(offset_rem(15, 7, 2, 9).unwrap(), (54, 105));
    }

    #[test]
    fn non_offset_skip_large_small() {
        assert_eq!(offset_rem(17, 13, 2, 0).unwrap(), (102, 221));
    }

    #[test]
    fn offset_large_number() {
        assert_eq!(
            offset_rem(166439, 19, 7, 70147).unwrap(),
            (1068781, 3162341)
        );
    }

    #[test]
    fn offset() {
        assert_eq!(offset_rem(221, 19, 3, 102).unwrap(), (3417, 4199));
    }

    #[test]
    fn chain() {
        let data = vec![Some(17), None, Some(13), Some(19)];
        assert_eq!(chain_offset(data.as_slice()).unwrap(), 3417)
    }
}
