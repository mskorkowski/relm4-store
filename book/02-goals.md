# Goals of the store library

- [ ] Window behavior on data changes
  - [ ] Window transitions
    - [x] Add new record
    - [ ] Slide
    - [ ] Remove
  - [ ] View follows transitions
- [ ] Relative scrolling
- [ ] Sorting
  - [x] Setting natural order
  - [ ] Store view order
- [ ] Grouping
- [ ] Trees
- [ ] Implementation of basic store types
  - [ ] In memory store
    - [ ] Sorting
      - [x] Natural order
      - [ ] Store view order
  - [ ] Http/rest store (as external crate)
  - [ ] Mongo store (as external crate)
- [ ] Detached view
  - [ ] Commit
- [ ] Filtering
- [ ] Reusable components
  - [ ] Pagination component
    - [x] Basics
    - [ ] Make it configurable and generally awesome
- [x] Pagination
- [x] View should care about records which are visible only
- [x] Generic view for the store
  - [x] Replaces relm4 factory

## User friendliness

These goals are making usage of the library easier

- [x] Book
  - [ ] Todo 1
  - [ ] Todo 2
  - [ ] Todo 3
  - [ ] Todo 4
- [ ] Examples
  - [x] Todo 1 - really simple todo
  - [x] Todo 2 - todo 1 with pagination
  - [x] Todo 4 - todo 2 with sorting
  - [x] Window behavior - todo 4 many times over
  
- [ ] rustdoc

## Missing things

- [ ] Remove records
- [ ] Reorder records
- [x] Add records
  - [x] At the beginning
  - [x] At the end
  - [x] Somewhere between the elements
- [x] Remove dependency to StoreMsg if things are not related to store
