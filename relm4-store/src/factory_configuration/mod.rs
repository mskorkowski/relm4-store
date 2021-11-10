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
pub trait FactoryConfiguration<Allocator=DefaultIdAllocator>: ViewModel<Widgets = Self::ContainerWidgets> 
where
    Allocator: TemporaryIdAllocator,
{
    /// Store type wich will be a backend for your data
    type Store: DataStore<Allocator>;
    /// Structure with widgets used by this component
    type RecordWidgets: Debug;
    /// Type of root widget in [FactoryConfiguration::RecordWidgets]
    type Root: WidgetExt;
    
    type View: FactoryView<Self::Root> + FactoryListView<Self::Root>;
    type Window: WindowBehavior;
    type ContainerWidgets: FactoryContainerWidgets<Self, Allocator>;

    // type Msg;
    // type ViewModel: ViewModel<Msg=Self::Msg>;
    type ParentViewModel: ViewModel;

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

    fn update(
        &mut self,
        msg: Self::Msg,
        sender: Sender<Self::Msg>,
    );

    fn init_view_model(parent_view_model: &Self::ParentViewModel, store_view: Rc<RefCell<StoreViewImplementation<Self, Allocator>>>) -> Self;

    fn position(
        model: <Self::Store as DataStore<Allocator>>::Record, 
        position: Position,
    ) -> <Self::View as FactoryView<Self::Root>>::Position;

    /// Get the outermost widget from the widgets.
    fn get_root(widgets: &Self::RecordWidgets) -> &Self::Root;
}

/// Trait describing what do we need from widgets to be usable for the [StoreViewComponent]
pub trait FactoryContainerWidgets<FactoryViewModel: FactoryConfiguration<Allocator, Widgets=Self, ContainerWidgets=Self>, Allocator=DefaultIdAllocator> 
where
    Allocator: TemporaryIdAllocator,
{
    type Root: std::fmt::Debug;

    fn init_view(
        view_model: &FactoryViewModel, 
        store_view: &StoreViewImplementation<FactoryViewModel, Allocator>,
        sender: Sender<<FactoryViewModel as ViewModel>::Msg>
    ) -> Self;
    
    fn view(
        &mut self, 
        view_model: &FactoryViewModel, 
        store_view: &StoreViewImplementation<FactoryViewModel, Allocator>,
        sender: Sender<<FactoryViewModel as ViewModel>::Msg>
    );
    
    fn root_widget(&self) -> Self::Root;

    fn connect_components(&self, _model: &FactoryViewModel, _components: &FactoryViewModel::Components) {}

    fn container_widget(&self) -> &<FactoryViewModel as FactoryConfiguration<Allocator>>::View;
}
