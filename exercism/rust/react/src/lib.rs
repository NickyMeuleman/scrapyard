use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct InputCell<T> {
    value: T,
}

struct ComputeCell<'a, T> {
    dependencies: Vec<CellID>,
    compute_func: Box<dyn 'a + Fn(&[T]) -> T>,
    prev_value: Option<T>,
    callbacks: HashMap<CallbackID, Box<dyn FnMut(T) + 'a>>,
}

pub struct Reactor<'a, T> {
    counter: usize,
    inputs: HashMap<InputCellID, InputCell<T>>,
    computes: HashMap<ComputeCellID, ComputeCell<'a, T>>,
}

impl<T> InputCell<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
}

impl<'a, T> ComputeCell<'a, T> {
    fn new(dependencies: &[CellID], compute_func: Box<dyn 'a + Fn(&[T]) -> T>) -> Self {
        Self {
            compute_func,
            dependencies: dependencies.to_vec(),
            callbacks: HashMap::new(),
            prev_value: None,
        }
    }
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            counter: 0,
            inputs: HashMap::new(),
            computes: HashMap::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        // create ID by using the counter, increment it so it's ready for the next user
        let id = InputCellID(self.counter);
        self.counter += 1;

        // create new inputcell and add it to the inputs
        let input = InputCell::new(initial);
        self.inputs.insert(id, input);

        id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: 'a + Fn(&[T]) -> T>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        // check if all dependencies exist
        for dep_id in dependencies {
            if match dep_id {
                CellID::Compute(id) => !self.computes.contains_key(id),
                CellID::Input(id) => !self.inputs.contains_key(id),
            } {
                return Err(*dep_id);
            }
        }

        // create ID by using the counter, increment it so it's ready for the next user
        let id = ComputeCellID(self.counter);
        self.counter += 1;

        // create new computecell and add it to the computes
        let compute = ComputeCell::new(dependencies, Box::new(compute_func));
        self.computes.insert(id, compute);

        // get value
        let value = self.value(CellID::Compute(id));

        // set value as prev_value
        self.computes.get_mut(&id).unwrap().prev_value = value;

        Ok(id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            // an inputcell holds a value directly
            CellID::Input(id) => self.inputs.get(&id).map(|cell| cell.value),
            // a computecell value has to be calculated
            CellID::Compute(id) => self.computes.get(&id).map(|cell| {
                // get function to calculate value of this cell
                let compute_func = &cell.compute_func;

                // get arguments the compute_function needs
                let deps: Vec<T> = cell
                    .dependencies
                    .iter()
                    .filter_map(|dep| self.value(*dep))
                    .collect();

                // call compute_func with those arguments to get the value of the cell
                compute_func(deps.as_slice())
            }),
        }
    }

    // Sets the value of the specified input cell.
    // Returns false if the cell does not exist.
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        // TODO: use iterator adapters instead of for loops. Tried but the borrow checker said "nah"

        // get the inputcell
        match self.inputs.get_mut(&id) {
            Some(cell) => {
                // set new_value in inputcell
                cell.value = new_value;

                // loop through every computecell and mark the ones that need updates
                let needs_updates: Vec<_> = self
                    .computes
                    .iter()
                    .filter_map(|(&compute_id, compute)| {
                        let new_value = self.value(CellID::Compute(compute_id));
                        if new_value != compute.prev_value {
                            Some((compute_id, new_value))
                        } else {
                            None
                        }
                    })
                    .collect();

                for (compute_id, value) in needs_updates {
                    // set prev_value to the changed value we found
                    let compute_cell = self.computes.get_mut(&compute_id).unwrap();
                    compute_cell.prev_value = value;

                    // call the callbacks for the changed cells with the new value
                    for callback in compute_cell.callbacks.values_mut() {
                        callback(value.unwrap())
                    }
                }

                true
            }
            None => false,
        }
    }

    // Adds a callback to the specified compute cell.
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    // Callbacks on input cells will not be tested.
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellID,
        callback: F,
    ) -> Option<CallbackID> {
        // create ID by using the counter, increment it so it's ready for the next user
        let callback_id = CallbackID(self.counter);
        self.counter += 1;

        // create new callback and add it to the callbacks of the computecell
        let compute = self.computes.get_mut(&id)?;
        compute.callbacks.insert(callback_id, Box::new(callback));

        Some(callback_id)
    }

    // Removes the specified callback, using an ID returned from add_callback.
    // Returns an Err if either the cell or callback does not exist.
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        // get computecell corresponding to function param: cell
        match self.computes.get_mut(&cell) {
            None => Err(RemoveCallbackError::NonexistentCell),
            // remove callback from computecell corresponding to function param: callback
            Some(cell) => match cell.callbacks.remove(&callback) {
                None => Err(RemoveCallbackError::NonexistentCallback),
                Some(_) => Ok(()),
            },
        }
    }
}
