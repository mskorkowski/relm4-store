mod impls;

use reexport::gtk;
use reexport::relm4;

use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use gtk::glib::Sender;
use gtk::prelude::WidgetExt;

use relm4::Model as ViewModel;
use relm4::factory::Factory;
use relm4::factory::FactoryListView;
use relm4::factory::FactoryPrototype;
use relm4::factory::FactoryView;

use record::DefaultIdAllocator;
use record::TemporaryIdAllocator;

use crate::DataStore;
use crate::StoreSize;
use crate::StoreView;
use crate::position::Position;
use crate::redraw_messages::RedrawMessages;
use crate::window::WindowBehavior;

/// Configuration of the [StoreViewComponent]
pub trait FactoryConfiguration<Allocator, StoreIdAllocator>
where
    Allocator: TemporaryIdAllocator,
    StoreIdAllocator: TemporaryIdAllocator,
{
    /// Store type which will be a backend for your data
    type Store: DataStore<Allocator, StoreIdAllocator>;
    /// StoreView type
    type StoreView: StoreView<Allocator, StoreIdAllocator, Record=<Self::Store as DataStore<Allocator, StoreIdAllocator>>::Record> + 
        Factory<Self::StoreView, Self::View> +
        FactoryPrototype<Factory=Self::StoreView, Msg=<Self::ViewModel as ViewModel>::Msg, Widgets=Self::RecordWidgets, Root=Self::Root, View=Self::View> + 
        Debug;

    /// Structure with widgets used by this component
    type RecordWidgets: Debug;
    /// Type of root widget in [FactoryConfiguration::RecordWidgets]
    /// 
    /// Same as [relm4::factory::FactoryPrototype::Root]
    type Root: WidgetExt;
    /// Type of widget to which record widgets will be attached to
    /// 
    /// Same as [relm4::factory::FactoryPrototype::View]
    type View: FactoryView<Self::Root> + FactoryListView<Self::Root>;

    /// Type describing how visible data window should behave in case of new data
    type Window: WindowBehavior;

    /// ViewModel of the component which will be handling store view
    type ViewModel: ViewModel;
    
    /// ViewModel of the parent component
    type ParentViewModel: ViewModel;

    /// Initialize store view
    fn init_store_view(store: Rc<RefCell<Self::Store>>, size: StoreSize, redraw_sender: Sender<RedrawMessages>) -> Self::StoreView;

    /// Creates instance of the [Self::RecordWidgets] responsible for displaying `record`
    /// at the `position`
    fn generate(
        record: &<Self::Store as DataStore<Allocator, StoreIdAllocator>>::Record,
        position: Position,
        sender: Sender<<Self::ViewModel as ViewModel>::Msg>,
    ) -> Self::RecordWidgets;

    /// Function called when record in store view is modified and you need to 
    /// synchronize the state of the view with data in the model
    fn update_record(
        model: <Self::Store as DataStore<Allocator, StoreIdAllocator>>::Record,
        position: Position,
        widgets: &Self::RecordWidgets,
    );

    /// Function called when component received a message
    fn update(
        view_model: &mut Self::ViewModel,
        msg: <Self::ViewModel as ViewModel>::Msg,
        sender: Sender<<Self::ViewModel as ViewModel>::Msg>,
    );

    /// Creates new instance of [FactoryConfiguration]
    /// 
    /// If you wish to use store view in widgets you must save it in your model
    fn init_view_model(parent_view_model: &Self::ParentViewModel, store_view: Rc<RefCell<Self::StoreView>>) -> Self::ViewModel;

    /// Returns position of record inside the widget
    /// 
    /// Useful for [gtk::Grid]
    fn position(
        model: <Self::Store as DataStore<Allocator, StoreIdAllocator>>::Record, 
        position: Position,
    ) -> <Self::View as FactoryView<Self::Root>>::Position;

    /// Get the outermost widget from the widgets.
    fn get_root(widgets: &Self::RecordWidgets) -> &Self::Root;
}

/// Trait describing what do we need from widgets to be usable for the [StoreViewComponent]
pub trait FactoryContainerWidgets<Configuration, Allocator, StoreIdAllocator> 
where
    Configuration: ?Sized + FactoryConfiguration<Allocator, StoreIdAllocator>,
    Allocator: TemporaryIdAllocator,
    StoreIdAllocator: TemporaryIdAllocator,
{
    /// Returns reference to the widget containing the records from the store view
    fn container_widget(&self) -> &<Configuration as FactoryConfiguration<Allocator, StoreIdAllocator>>::View;
}

/// Extra methods required by components embedded in StoreViewComponent
pub trait StoreViewInnerComponent<ParentModel: ?Sized + ViewModel>: relm4::Components<ParentModel> {
    /// This method is called when the store or store view was updated
    /// 
    /// Implementation of this method should send appropriate messages to the components defined in
    /// Self, so they can update themselves
    fn on_store_update(&mut self);
}