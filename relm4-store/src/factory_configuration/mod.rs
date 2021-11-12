mod impls;

use record::DefaultIdAllocator;
use record::TemporaryIdAllocator;
use reexport::gtk;
use reexport::relm4;

use std::cell::RefCell;
use std::fmt::Debug;
use gtk::prelude::WidgetExt;
use std::rc::Rc;

use gtk::glib::Sender;

use relm4::Model as ViewModel;
use relm4::factory::FactoryListView;
use relm4::factory::FactoryView;

use crate::store_view_implementation::StoreViewImplementation;
use crate::window::WindowBehavior;

use super::DataStore;
use super::position::Position;

/// Configuration of the [StoreViewComponent]
pub trait FactoryConfiguration<Widgets, Components, Allocator=DefaultIdAllocator>: ViewModel<Widgets = Widgets, Components=Components> 
where
    Widgets: FactoryContainerWidgets<Self, Components, Allocator>,
    Allocator: TemporaryIdAllocator,
    Widgets: ?Sized,
    Components: StoreViewInnerComponent<Self>
{
    /// Store type which will be a backend for your data
    type Store: DataStore<Allocator>;
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

    // type ContainerWidgets: FactoryContainerWidgets<Self, Allocator>;

    // type Msg;
    // type ViewModel: ViewModel<Msg=Self::Msg>;

    /// ViewModel of the parent component
    type ParentViewModel: ViewModel;

    /// Creates instance of the [Self::RecordWidgets] responsible for displaying `record`
    /// at the `position`
    fn generate(
        record: &<Self::Store as DataStore<Allocator>>::Record,
        position: Position,
        sender: Sender<Self::Msg>,
    ) -> Self::RecordWidgets;

    /// Function called when record in store view is modified and you need to 
    /// synchronize the state of the view with data in the model
    fn update_record(
        model: <Self::Store as DataStore<Allocator>>::Record,
        position: Position,
        widgets: &Self::RecordWidgets,
    );

    /// Function called when component received a message
    fn update(
        &mut self,
        msg: Self::Msg,
        sender: Sender<Self::Msg>,
    );

    /// Creates new instance of [FactoryConfiguration]
    /// 
    /// If you wish to use store view in widgets you must save it in your model
    fn init_view_model(parent_view_model: &Self::ParentViewModel, store_view: Rc<RefCell<StoreViewImplementation<Widgets, Self, Components, Allocator>>>) -> Self;

    /// Returns position of record inside the widget
    /// 
    /// Useful for [gtk::Grid]
    fn position(
        model: <Self::Store as DataStore<Allocator>>::Record, 
        position: Position,
    ) -> <Self::View as FactoryView<Self::Root>>::Position;

    /// Get the outermost widget from the widgets.
    fn get_root(widgets: &Self::RecordWidgets) -> &Self::Root;
}

/// Trait describing what do we need from widgets to be usable for the [StoreViewComponent]
pub trait FactoryContainerWidgets<FactoryViewModel: FactoryConfiguration<Self, Components, Allocator>, Components, Allocator=DefaultIdAllocator> 
where
    Allocator: TemporaryIdAllocator,
    Components: StoreViewInnerComponent<FactoryViewModel>,
{
    /// Type of the root widget for this widgets
    type Root: std::fmt::Debug;

    /// Creates new instance of this
    fn init_view(
        view_model: &FactoryViewModel, 
        sender: Sender<<FactoryViewModel as ViewModel>::Msg>
    ) -> Self;
    
    /// Update the view to represent the updated model.
    fn view(
        &mut self, 
        view_model: &FactoryViewModel, 
        sender: Sender<<FactoryViewModel as ViewModel>::Msg>
    );
    
    /// Returns reference to the widget in the root of this
    fn root_widget(&self) -> Self::Root;

    /// Connects components to this widgets
    fn connect_components(&self, _model: &FactoryViewModel, _components: &FactoryViewModel::Components) {}

    /// Returns reference to the widget containing the records from the store view
    fn container_widget(&self) -> &<FactoryViewModel as FactoryConfiguration<Self, Components, Allocator>>::View;
}

/// Extra methods required by components embedded in StoreViewComponent
pub trait StoreViewInnerComponent<ParentModel: ?Sized + ViewModel>: relm4::Components<ParentModel> {
    /// This method is called when the store or store view was updated
    /// 
    /// Implementation of this method should send appropriate messages to the components defined in
    /// Self, so they can update themselves
    fn on_store_update(&mut self);
}