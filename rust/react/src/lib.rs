#![feature(test)]
extern crate test;
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicUsize, Ordering};
static COUNTER: AtomicUsize = AtomicUsize::new(1);
fn fetch_add() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InputCellId(usize);
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ComputeCellId(usize);
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CallbackId(usize);
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}
#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}
#[derive(Default)]
pub struct Reactor<'a, T> {
    input_cells: HashMap<InputCellId, T>,
    compute_cells: HashMap<ComputeCellId, ComputeCell<'a, T>>,
}
type Compute<'a, T> = dyn 'a + Fn(&[T]) -> T;
type Callback<'a, T> = dyn 'a + FnMut(T);
struct ComputeCell<'a, T> {
    value: T,
    compute: Box<Compute<'a, T>>,
    dependencies: Vec<CellId>,
    callbacks: HashMap<CallbackId, Box<Callback<'a, T>>>,
}
impl<'a, T: Copy + PartialEq + Default> Reactor<'a, T> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    pub fn create_input(&mut self, value: T) -> InputCellId {
        let input_cell_id = InputCellId(fetch_add());
        self.input_cells.insert(input_cell_id, value);
        input_cell_id
    }
    pub fn create_compute(
        &mut self,
        dependencies: &[CellId],
        compute: impl 'a + Fn(&[T]) -> T,
    ) -> Result<ComputeCellId, CellId> {
        for cell_id in dependencies {
            match cell_id {
                CellId::Input(input_cell_id) if !self.input_cells.contains_key(input_cell_id) => {
                    return Err(*cell_id)
                }
                CellId::Compute(compute_cell_id)
                    if !self.compute_cells.contains_key(compute_cell_id) =>
                {
                    return Err(*cell_id)
                }
                _ => (),
            }
        }
        let arguments: Vec<T> = dependencies
            .iter()
            .map(|cell_id| self.value(*cell_id).unwrap())
            .collect();
        let compute_cell = ComputeCell {
            value: compute(arguments.as_slice()),
            compute: Box::new(compute),
            dependencies: dependencies.to_vec(),
            callbacks: HashMap::new(),
        };
        let compute_cell_id = ComputeCellId(fetch_add());
        self.compute_cells.insert(compute_cell_id, compute_cell);
        Ok(compute_cell_id)
    }
    #[must_use]
    pub fn value(&self, cell_id: CellId) -> Option<T> {
        match cell_id {
            CellId::Input(input_cell_id) => self.input_cells.get(&input_cell_id).copied(),
            CellId::Compute(compute_cell_id) => self
                .compute_cells
                .get(&compute_cell_id)
                .map(|compute_cell| compute_cell.value),
        }
    }
    fn propagate(
        &mut self,
        input_cell_id: InputCellId,
        invoke_callbacks: &mut HashMap<ComputeCellId, T>,
    ) {
        let mut dirty_compute_cell_ids: VecDeque<ComputeCellId> = self
            .compute_cells
            .iter()
            .filter_map(|(compute_cell_id, compute_cell)| {
                if compute_cell
                    .dependencies
                    .contains(&CellId::Input(input_cell_id))
                {
                    Some(compute_cell_id)
                } else {
                    None
                }
            })
            .copied()
            .collect();
        while let Some(dirty_compute_cell_id) = dirty_compute_cell_ids.pop_front() {
            let dirty_compute_cell = self.compute_cells.get(&dirty_compute_cell_id).unwrap();
            let arguments: Vec<T> = dirty_compute_cell
                .dependencies
                .iter()
                .map(|&cell_id| self.value(cell_id).unwrap())
                .collect();
            let compute_cell_value = (dirty_compute_cell.compute)(arguments.as_slice());
            let dirty_compute_cell = self.compute_cells.get_mut(&dirty_compute_cell_id).unwrap();
            if dirty_compute_cell.value != compute_cell_value {
                dirty_compute_cell.value = compute_cell_value;
                invoke_callbacks
                    .entry(dirty_compute_cell_id)
                    .and_modify(|value| *value = compute_cell_value)
                    .or_insert(compute_cell_value);
            }
            for (compute_cell_id, compute_cell) in &self.compute_cells {
                if compute_cell
                    .dependencies
                    .contains(&CellId::Compute(dirty_compute_cell_id))
                {
                    dirty_compute_cell_ids.push_back(*compute_cell_id);
                }
            }
        }
    }
    fn notify(&mut self, invoke_callbacks: &HashMap<ComputeCellId, T>) {
        for (compute_cell_id, value) in invoke_callbacks {
            let compute_cell = self.compute_cells.get_mut(compute_cell_id).unwrap();
            for callback in compute_cell.callbacks.values_mut() {
                callback(*value);
            }
        }
    }
    pub fn set_value(&mut self, input_cell_id: InputCellId, input_cell_value: T) -> bool {
        let Some(value) = self.input_cells.get_mut(&input_cell_id) else {
            return false;
        };
        if *value != input_cell_value {
            *value = input_cell_value;
            let mut invoke_callbacks = HashMap::new();
            self.propagate(input_cell_id, &mut invoke_callbacks);
            self.notify(&invoke_callbacks);
        }
        true
    }
    pub fn add_callback(
        &mut self,
        compute_cell_id: ComputeCellId,
        callback: impl 'a + FnMut(T),
    ) -> Option<CallbackId> {
        self.compute_cells
            .get_mut(&compute_cell_id)
            .map(|compute_cell| {
                let callback_id = CallbackId(fetch_add());
                compute_cell
                    .callbacks
                    .insert(callback_id, Box::new(callback));
                callback_id
            })
    }
    pub fn remove_callback(
        &mut self,
        compute_cell_id: ComputeCellId,
        callback_id: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        let Some(compute_cell) = self.compute_cells.get_mut(&compute_cell_id) else {
            return Err(RemoveCallbackError::NonexistentCell);
        };
        match compute_cell.callbacks.remove(&callback_id) {
            Some(_) => Ok(()),
            None => Err(RemoveCallbackError::NonexistentCallback),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    #[bench]
    fn adder_with_boolean_values(b: &mut test::Bencher) {
        b.iter(|| {
            let mut reactor = Reactor::new();
            let a = reactor.create_input(false);
            let b = reactor.create_input(false);
            let carry_in = reactor.create_input(false);
            let a_xor_b = reactor
                .create_compute(&[CellId::Input(a), CellId::Input(b)], |v| v[0] ^ v[1])
                .unwrap();
            let sum = reactor
                .create_compute(&[CellId::Compute(a_xor_b), CellId::Input(carry_in)], |v| {
                    v[0] ^ v[1]
                })
                .unwrap();
            let a_xor_b_and_cin = reactor
                .create_compute(&[CellId::Compute(a_xor_b), CellId::Input(carry_in)], |v| {
                    v[0] && v[1]
                })
                .unwrap();
            let a_and_b = reactor
                .create_compute(&[CellId::Input(a), CellId::Input(b)], |v| v[0] && v[1])
                .unwrap();
            let carry_out = reactor
                .create_compute(
                    &[CellId::Compute(a_xor_b_and_cin), CellId::Compute(a_and_b)],
                    |v| v[0] || v[1],
                )
                .unwrap();
            let tests = &[
                (false, false, false, false, false),
                (false, false, true, false, true),
                (false, true, false, false, true),
                (false, true, true, true, false),
                (true, false, false, false, true),
                (true, false, true, true, false),
                (true, true, false, true, false),
                (true, true, true, true, true),
            ];
            for &(aval, bval, cinval, expected_cout, expected_sum) in tests {
                assert!(reactor.set_value(a, aval));
                assert!(reactor.set_value(b, bval));
                assert!(reactor.set_value(carry_in, cinval));
                assert_eq!(reactor.value(CellId::Compute(sum)), Some(expected_sum));
                assert_eq!(
                    reactor.value(CellId::Compute(carry_out)),
                    Some(expected_cout)
                );
            }
        });
    }
    #[bench]
    fn bench_3(b: &mut Bencher) {
        let mut reactor = Reactor::new();
        let input = reactor.create_input(0);
        let interim = reactor
            .create_compute(&[CellId::Input(input)], |v| v[0] * 2)
            .unwrap();
        let _result = reactor.create_compute(&[CellId::Compute(interim)], |v| v[0] * 2);
        let mut it = vec![1, 2, 3].into_iter().cycle();
        b.iter(|| reactor.set_value(input, it.next().unwrap_or(0)));
    }
    #[bench]
    fn bench_30(b: &mut Bencher) {
        let mut reactor = Reactor::new();
        let input = reactor.create_input(0);
        let mut interim = reactor
            .create_compute(&[CellId::Input(input)], |v| v[0] * 2)
            .unwrap();
        for _ in 0..28 {
            interim = reactor
                .create_compute(&[CellId::Compute(interim)], |v| v[0] * 2)
                .unwrap();
        }
        let _result = reactor.create_compute(&[CellId::Compute(interim)], |v| v[0] * 2);
        let mut it = vec![1, 2, 3].into_iter().cycle();
        b.iter(|| reactor.set_value(input, it.next().unwrap_or(0)));
    }
    #[bench]
    fn bench_9_5(b: &mut Bencher) {
        let mut reactor = Reactor::new();
        let input = reactor.create_input(0);
        let interim = reactor
            .create_compute(&[CellId::Input(input)], |v| v[0] * 2)
            .unwrap();
        let mut computes = vec![CellId::Compute(interim)];
        for _ in 0..6 {
            let id = reactor
                .create_compute(computes.as_slice(), |v| v.iter().sum::<i32>())
                .unwrap();
            computes.push(CellId::Compute(id))
        }
        let _result = reactor.create_compute(computes.as_slice(), |v| v[0] * 2);
        let mut it = vec![1, 2, 3].into_iter().cycle();
        b.iter(|| reactor.set_value(input, it.next().unwrap_or(0)));
    }
}
