# relm4-store-backend-dummy

This crate contains an implementation of the data store which is intended for testing/debugging code using store and store views.

Punch line
> DummyBackend is a data store in which you manually control transition between states

As the consequence of "manual control" you are responsible on the correctness of the transitions. If you add some records during transition of the state but you don't provide the appropriate messages to be sent to store view please do not expect that store view is going to behave in any sane way.

## Rules to follow while contributing here

1. Since it's test/debug helper facility any updates/fixes/features here should come with appropriate test cases. It's extremely frustrating when your tests are broken because the tools you use are not working as advertised. It's not like 100% code coverage is required. The closer we are to 100% just better and for this crate pr's will be checked for this really carefully. Currently many tests are not comprehensive. For example they test if the returned vector has expected number of elements but not the elements itself.
2. Let's keep implementation predictable to the extreme. Data store publishes it's sender so in theory you could drive the data store using it. Let's be frank if you depend on the code sending a message or not to the data store while you test a data store and use store view to observe the changes this would be nightmare. Testing async code and senders/receivers are just that is complex enough without side effects spreading into your data store. Defining what changed and what to send to the view every time is lots of work, but it gives you a full control and predictability.
3. Let's keep implementation as simple as possible. Simpler it is, less issues are there and smaller intellectual burden it is.
4. Scout rule applies, leave world a better place

## What would be nice to have?

- Ability to check incoming messages to the data store and provide a way to tell what order of events is acceptable.
- Create a way to check returned content from the tested backend

### Checking order of incoming messages

Currently we just ignore incoming messages. Checking them in which order they were sent would be better.

For example if you expect Add and Remove events to be sent to empty store you would expect them in order of `Add` first and `Remove` later (in most cases). Since they can be interpreted as user adding and later removing record. If they come other way around `Remove` will be considered noop by the store since there is no record to remove and add will leave one record in the store which would probably be a bad idea in such a case.

### Create a way to check returned content from the tested backend

Tests for `DummyBackend::get_range` are currently checking if returned collection has expected size but they are not checking if the returned vector has proper content.

This is something potentially against rule `1`. Current implementation is simple enough that I'm not so worried about. Nether the less it's definitely on the todo list to make it happen.
