Reimplementation of rust's std library, intended primarily for learning purposes.

### Implementation list

* core
- [x] iterator (library/core/src/iter/traits/iterator.rs)
- [x] into_iter
- [x] from_iterator
- [ ] borrow (library/core/src/iter/traits/iterator.rs)
- [x] mem (library/core/src/mem/mod.rs)
  - [ ] non_null
- [ ] pin
- [ ] FnMut

* alloc
- [ ] Box
- [ ] Rc
- [ ] RawVec
- [ ] Borrow (Cow)
- [ ] Vec
- [ ] String

* std
- [ ] Vec (wip)
- [ ] Option
- [x] cell
- [x] refcell
- [ ] Mutex
- [ ] Index
