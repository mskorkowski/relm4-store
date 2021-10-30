use reexport::gtk;
use reexport::relm4;

use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

use gtk::glib::Sender;
use relm4::factory::Factory;
use relm4::factory::FactoryPrototype;
use relm4::factory::FactoryView;

use model::Identifiable;

use super::DataStore;
use super::DataStoreBase;
use super::DataStoreListenable;
use super::Handler;
use super::HandlerWrapper;
use super::FactoryBuilder;
use super::IdentifiableStore;
use super::math;
use super::Position;
use super::RecordWithLocation;
use super::StoreId;
use super::StoreMsg;
use super::StoreSize;
use super::StoreView;
use super::StoreViewImpl;
use super::StoreViewImplHandler;

pub struct StoreViewInterface<Builder> 
where
    Builder: FactoryBuilder + 'static,
{
    view: Rc<RefCell<StoreViewImpl<Builder>>>
}

impl<Builder> StoreViewInterface<Builder> 
where 
    Builder: FactoryBuilder + 'static,
{
    pub fn new(store: Rc<RefCell<Builder::Store>>, size: StoreSize) -> Self {
        let view = Rc::new(RefCell::new(StoreViewImpl::new(store.clone(), size.items())));
        let view_id = view.borrow().get_id();
        let weak_view = Rc::downgrade(&view);

        let s: RefMut<Builder::Store> = store.borrow_mut();

        s.listen(
            view_id.transfer(),
            Box::new(
                StoreViewImplHandler::new(weak_view),
            )
        );

        Self {
            view,
        }
    }
}

impl<Builder> Identifiable for StoreViewInterface<Builder>
where
    Builder: 'static + FactoryBuilder,
{
    type Id = StoreId<Self>;

    fn get_id(&self) -> Self::Id {
        self.view.borrow().get_id().transfer()
    }
}

impl<Builder> IdentifiableStore for StoreViewInterface<Builder>
where
    Builder: 'static + FactoryBuilder,
{}

impl<Builder> DataStoreBase for StoreViewInterface<Builder>
where
    Builder: 'static + FactoryBuilder,
{
    type Model = <Builder::Store as DataStoreBase>::Model;

    fn inbox(&self, msg: StoreMsg<<Self as DataStoreBase>::Model>) {
        self.view.borrow().inbox(msg);
    }

    fn len(&self) -> usize { 
        self.view.borrow().len()
    }

    fn is_empty(&self) -> bool { 
        self.view.borrow().is_empty()
    }

    fn get(&self, id: &<Self::Model as Identifiable>::Id) -> Option<Self::Model> { 
        self.view.borrow().get(id)
     }

    fn get_range(&self, range: &math::Range) -> std::vec::Vec<Self::Model> {
        self.view.borrow().get_range(range)
    }
}

impl<Builder> DataStoreListenable for StoreViewInterface<Builder>
where
    Builder: 'static + FactoryBuilder,
{
    fn listen(&self, handler_ref: StoreId<Self>,  handler: std::boxed::Box<(dyn Handler<Self> + 'static)>) { 
        self.view.borrow_mut().listen(
            handler_ref.transfer(),
            HandlerWrapper::from(handler)
        )
     }

    fn unlisten(&self, id: StoreId<Self>) { 
        self.view.borrow_mut().unlisten(id.transfer())
    }
}

impl<Builder> DataStore for StoreViewInterface<Builder>
where
    Builder: 'static + FactoryBuilder,
{}

impl<Builder> StoreView for StoreViewInterface<Builder>
where
    Builder: 'static + FactoryBuilder,
{
    type Builder = Builder;

    fn window_size(&self) -> usize {
        self.view.borrow().window_size()
    }

    fn get_view_data(&self) -> Vec<RecordWithLocation<Self::Model>> {
        self.view.borrow().get_view_data()
    }

    fn first_page(&self) {
        self.view.borrow().first_page();
    }

    fn prev_page(&self) {
        self.view.borrow().prev_page();
    }

    fn next_page(&self) {
        self.view.borrow().next_page();
    }

    fn last_page(&self) {
        self.view.borrow().last_page();
    }

    fn get_window(&self) -> math::Range {
        self.view.borrow().get_window()
    }

    fn get_position(&self, id: &<Self::Model as Identifiable>::Id) -> Option<Position> {
        self.view.borrow().get_position(id)
    }
}

impl<Builder> FactoryPrototype for StoreViewInterface<Builder>
where
    Builder: FactoryBuilder + 'static,
{
    type Factory = Self;
    type Msg = Builder::Msg;
    type Widgets = Builder::Widgets;
    type Root = Builder::Root;
    type View = Builder::View;

    fn generate(
        &self,
        key: &<Self::Factory as Factory<Self, Self::View>>::Key,
        sender: Sender<Builder::Msg>,
    ) -> Self::Widgets {
        let model = self.view.borrow().get(key).expect("Key doesn't point to the model in the store while generating! WTF?");
        let position = self.get_position(&model.get_id()).expect("Unsynchronized view with store! WTF?");
        Builder::generate(&model, position, sender)
    }

    /// Set the widget position upon creation, useful for [`gtk::Grid`] or similar.
    fn position(
        &self,
        key: &<Self::Factory as Factory<Self, Self::View>>::Key,
    ) -> <Self::View as FactoryView<Self::Root>>::Position {
        let model = self.view.borrow().get(key).expect("Key doesn't point to the model in the store while positioning! WTF?");
        let position = self.get_position(&model.get_id()).expect("Unsynchronized view with store! WTF?");
        Builder::position(model, position)
    }

    /// Function called when self is modified.
    fn update(
        &self,
        key: &<Self::Factory as Factory<Self, Self::View>>::Key,
        widgets: &Self::Widgets,
    ) {
        let model = self.view.borrow().get(key).expect("Key doesn't point to the model in the store while updating! WTF?");
        let position = self.get_position(&model.get_id()).expect("Unsynchronized view with store! WTF?");
        Builder::update(model, position, widgets)
    }

    /// Get the outermost widget from the widgets.
    fn get_root(widgets: &Self::Widgets) -> &Self::Root {
        Builder::get_root(widgets)
    }
}

impl<Builder> Factory<StoreViewInterface<Builder>, Builder::View> for StoreViewInterface<Builder>
where
    Builder: FactoryBuilder + 'static,
{
    type Key = <<Builder::Store as DataStoreBase>::Model as Identifiable>::Id;

    fn generate(&self, view: &Builder::View, sender: Sender<Builder::Msg>) {
        let view_impl = self.view.borrow();
        view_impl.generate(view, sender);
    }
}