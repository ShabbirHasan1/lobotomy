use crate::common::intrinsics::*;
use crate::common::utils::round_to_tick_size;

use arrayvec::ArrayVec;

#[derive(Debug)]
pub struct L2Book<const N: usize, const REVERSE: bool> {
    levels: ArrayVec<f64, N>,
    tick_size: f64,
}

impl<const N: usize, const REVERSE: bool> L2Book<N, REVERSE> {
    pub fn new(tick_size: f64) -> Self {
        L2Book {
            levels: ArrayVec::new(),
            tick_size,
        }
    }

    /// Cases:
    ///
    /// - Amount > 0 (Upsert):
    ///     1. Find the position for insertion/update. If the position is beyond the top (None), then take no action.
    ///     2. If we find the exact price, simply return.
    ///     3. Shift to the right from the insertion position.
    ///     4. Insert the new price.
    ///
    /// - Amount == 0 (Delete):
    ///     1. Find the position for the price. If the position is beyond the top (None), take no action.
    ///        Also, find the position of the worst price in the top.
    ///     2. If `top[pos] != price`, return.
    ///     3. If the position of the worst price is None, it means the top is empty - return.
    ///     4. When we delete the price, there will be a shift to the left,
    ///        leaving an empty spot at the position of the worst price. Therefore, we need to ask PriceMap for the next worst price with amount > 0.
    ///     5. Insert the next worst price in the empty spot.
    #[inline(always)]
    pub fn update(&mut self, px: f64, amt: f64, get_next_worst_px: impl Fn(f64) -> Option<f64>) {
        let px = round_to_tick_size(px, self.tick_size);
        let mut px_pos_opt = self.levels.is_empty().then_some(0);
        let mut worst_top_pos_opt = None;

        for (idx, other_px) in self.levels.iter().enumerate() {
            worst_top_pos_opt = Some(idx);

            if px_pos_opt.is_none() && Self::comparator(px, *other_px) {
                px_pos_opt = Some(idx);
            }
        }

        let px_pos = match px_pos_opt {
            None => return,
            Some(pos) => pos,
        };

        if amt == 0.0 {
            let worst_px = match worst_top_pos_opt {
                None => return,
                Some(pos) => self.levels[pos],
            };

            if self.levels[px_pos] != px {
                return;
            }

            self.levels.drain(px_pos..=px_pos);

            let next_worst_px = match get_next_worst_px(worst_px) {
                None => return,
                Some(next_worst_px) => round_to_tick_size(next_worst_px, self.tick_size),
            };

            self.levels.push(next_worst_px);

            return;
        }

        if unlikely(self.levels.is_empty()) {
            self.levels.push(px);
            return;
        }

        if self.levels[px_pos] == px {
            return;
        }

        if likely(self.levels.len() == N) {
            self.levels.pop();
        }

        self.levels.insert(px_pos, px);
    }

    #[inline(always)]
    fn comparator(a: f64, b: f64) -> bool {
        if REVERSE {
            a >= b
        } else {
            a <= b
        }
    }

    #[inline(always)]
    pub fn levels(&self) -> &[f64] {
        &self.levels
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.levels.clear();
    }
}