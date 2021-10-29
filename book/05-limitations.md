# Limitations

Whenever you update a store you must send an empty message to the all components using views of your store. This is related to how the relm4 `update - redraw` event loop works.
