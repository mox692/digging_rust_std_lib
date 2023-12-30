pub struct UnsafeCell<T> {
    value: T,
}

impl<T> UnsafeCell<T> {
    #[inline(always)]
    pub fn new(v: T) -> Self {
        UnsafeCell { value: v }
    }

    pub fn into_inner(self) -> T {
        self.value
    }

    pub fn get(&self) -> *mut T {
        self as *const UnsafeCell<T> as *const T as *mut T
    }
}

impl<T: Clone> Clone for UnsafeCell<T> {
    fn clone(&self) -> Self {
        UnsafeCell {
            value: self.value.clone(),
        }
    }
}
impl<T: Copy> Copy for UnsafeCell<T> {}

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(v: T) -> Self {
        Cell {
            value: UnsafeCell::new(v),
        }
    }

    pub fn replace(&self, val: T) -> T {
        std::mem::replace(unsafe { &mut *self.value.get() }, val)
    }

    pub fn set(&self, val: T) {
        self.replace(val);
    }

    pub fn into_inner(self) -> T {
        self.value.value
    }
}

impl<T: Clone> Clone for Cell<T> {
    fn clone(&self) -> Self {
        Cell {
            value: self.value.clone(),
        }
    }
}

impl<T: Copy> Cell<T> {
    // Replicate inner value by clone, and return that value.
    pub fn get(&self) -> T {
        self.value.value.clone()
    }
}
impl<T: Copy> Copy for Cell<T> {}

#[derive(Copy, Clone)]
enum RefState {
    Initialized,
    ImmutableRef(i32),
    MutableRef,
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    ref_state: Cell<RefState>,
}

impl<'b, T> RefCell<T> {
    pub fn new(value: T) -> Self {
        RefCell {
            value: UnsafeCell::new(value),
            ref_state: Cell::new(RefState::Initialized),
        }
    }

    pub fn borrow(&'b self) -> &'b T {
        let next_state = match self.ref_state.into_inner() {
            RefState::Initialized => RefState::ImmutableRef(1),
            RefState::ImmutableRef(n) => RefState::ImmutableRef(n + 1),
            RefState::MutableRef => panic!("err"),
        };
        self.ref_state.set(next_state);

        &self.value.value
    }

    pub fn borrow_mut(&'b self) -> &'b mut T {
        let next_state = match self.ref_state.into_inner() {
            RefState::Initialized => RefState::MutableRef,
            RefState::ImmutableRef(_) => panic!("errr"),
            RefState::MutableRef => panic!("errr"),
        };
        self.ref_state.set(next_state);

        unsafe { &mut *self.value.get() }
    }
}

impl<T: PartialEq> PartialEq for RefCell<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value.value == other.value.value
    }
}
