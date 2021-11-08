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
use super::DataStoreBase;
use super::position::Position;


pub trait FactoryBuilder: ViewModel<Widgets = Self::ContainerWidgets> {
    type Store: DataStore;
    type RecordWidgets: Debug;
    type Root: WidgetExt;
    type View: FactoryView<Self::Root> + FactoryListView<Self::Root>;
    type Window: WindowBehavior;
    type ContainerWidgets: FactoryContainerWidgets<Self>;

    // type Msg;
    // type ViewModel: ViewModel<Msg=Self::Msg>;
    type ParentViewModel: ViewModel;

    fn generate(
        record: &<Self::Store as DataStoreBase>::Model,
        position: Position,
        sender: Sender<Self::Msg>,
    ) -> Self::RecordWidgets;

    /// Function called when record in store view is modified and you need to 
    /// synchronize the state of the view with data in the model
    fn update_record(
        model: <Self::Store as DataStoreBase>::Model,
        position: Position,
        widgets: &Self::RecordWidgets,
    );

    fn update(
        &mut self,
        msg: Self::Msg,
        sender: Sender<Self::Msg>,
    );

    fn init_view_model(parent_view_model: &Self::ParentViewModel, store_view: Rc<RefCell<StoreViewImplementation<Self>>>) -> Self;

    fn position(
        model: <Self::Store as DataStoreBase>::Model, 
        position: Position,
    ) -> <Self::View as FactoryView<Self::Root>>::Position;

    /// Get the outermost widget from the widgets.
    fn get_root(widgets: &Self::RecordWidgets) -> &Self::Root;
}

pub trait FactoryContainerWidgets<FactoryViewModel: FactoryBuilder<Widgets=Self, ContainerWidgets=Self>> {
    type Root: std::fmt::Debug;

    fn init_view(
        view_model: &FactoryViewModel, 
        store_view: &StoreViewImplementation<FactoryViewModel>, 
        sender: Sender<<FactoryViewModel as ViewModel>::Msg>
    ) -> Self;
    
    fn view(
        &mut self, 
        view_model: &FactoryViewModel, 
        store_view: &StoreViewImplementation<FactoryViewModel>,
        sender: Sender<<FactoryViewModel as ViewModel>::Msg>
    );
    
    fn root_widget(&self) -> Self::Root;

    fn connect_components(&self, _model: &FactoryViewModel, _components: &FactoryViewModel::Components) {}

    fn container_widget(&self) -> &<FactoryViewModel as FactoryBuilder>::View;
}