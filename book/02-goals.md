# Goals of the store library

- [ ] Window behavior on data changes
  - [ ] Window transitions
    - [x] Add new record
    - [x] Slide
    - [ ] Reorder
    - [x] Remove
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
- [ ] Generic view for the store
  - [x] Replaces relm4 factory
  - [ ] Event propagation to the listeners of the view

## Missing things

- [ ] Reorder records, for now it triggers reload on the store
