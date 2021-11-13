use record::TemporaryIdAllocator;
use reexport::relm4;

use std::cell::Ref;

use relm4::Model as ViewModel;
use relm4::Sender;
use relm4::factory::Factory;
use relm4::factory::FactoryView;
use relm4::factory::FactoryPrototype;

use record::Id;
use record::Record;

use crate::DataStore;
use crate::StoreView;
use crate::FactoryConfiguration;
use crate::StoreViewImplementation;


impl<Configuration, Allocator> FactoryPrototype for StoreViewImplementation<Configuration, Allocator>
where
    Configuration: ?Sized + FactoryConfiguration<Allocator> + 'static,
    <Configuration::ViewModel as ViewModel>::Widgets: relm4::Widgets<Configuration::ViewModel, Configuration::ParentViewModel>,
    Allocator: TemporaryIdAllocator + 'static,
{
    type Factory = Self;
    type Msg = <Configuration::ViewModel as ViewModel>::Msg;
    type Widgets = Configuration::RecordWidgets;
    type Root = Configuration::Root;
    type View = Configuration::View;

    fn generate(
        &self,
        key: &<Self::Factory as Factory<Self, Self::View>>::Key,
        sender: Sender<<Configuration::ViewModel as ViewModel>::Msg>,
    ) -> Self::Widgets {
        let model = self.get(key).expect("Key doesn't point to the model in the store while generating! WTF?");
        let position = self.get_position(&model.get_id()).expect("Unsynchronized view with store! WTF?");
        Configuration::generate(&model, position, sender)
    }

    /// Set the widget position upon creation, useful for [`gtk::Grid`] or similar.
    fn position(
        &self,
        key: &<Self::Factory as Factory<Self, Self::View>>::Key,
    ) -> <Self::View as FactoryView<Self::Root>>::Position {
        let model = self.get(key).expect("Key doesn't point to the model in the store while positioning! WTF?");
        let position = self.get_position(&model.get_id()).expect("Unsynchronized view with store! WTF?");
        Configuration::position(model, position)
    }

    /// Function called when self is modified.
    fn update(
        &self,
        key: &<Self::Factory as Factory<Self, Self::View>>::Key,
        widgets: &Self::Widgets,
    ) {
        let model = self.get(key).expect("Key doesn't point to the model in the store while updating! WTF?");
        let position = self.get_position(&model.get_id()).expect("Unsynchronized view with store! WTF?");
        <Configuration as FactoryConfiguration<Allocator>>::update_record(model, position, widgets)
    }

    /// Get the outermost widget from the widgets.
    fn get_root(widgets: &Self::Widgets) -> &Self::Root {
        Configuration::get_root(widgets)
    }
}

impl<Configuration, Allocator> Factory<StoreViewImplementation<Configuration, Allocator>, Configuration::View> for StoreViewImplementation<Configuration, Allocator>
where
    Configuration: ?Sized + FactoryConfiguration<Allocator> + 'static,
    <Configuration::ViewModel as ViewModel>::Widgets: relm4::Widgets<Configuration::ViewModel, Configuration::ParentViewModel>,
    Allocator: TemporaryIdAllocator + 'static,
{
    type Key = Id<<Configuration::Store as DataStore<Allocator>>::Record>;

    fn generate(&self, view: &Configuration::View, sender: Sender<<Configuration::ViewModel as ViewModel>::Msg>) {
        self.view(view, sender);
    }
}

impl<Configuration, Allocator> Factory<StoreViewImplementation<Configuration, Allocator>, Configuration::View> for Ref<'_, StoreViewImplementation<Configuration, Allocator>>
where
    Configuration: ?Sized + FactoryConfiguration<Allocator> + 'static,
    <Configuration::ViewModel as ViewModel>::Widgets: relm4::Widgets<Configuration::ViewModel, Configuration::ParentViewModel>,
    Allocator: TemporaryIdAllocator + 'static,
{
    type Key = Id<<Configuration::Store as DataStore<Allocator>>::Record>;

    fn generate(&self, view: &Configuration::View, sender: Sender<<Configuration::ViewModel as ViewModel>::Msg>) {
        let me: &StoreViewImplementation<Configuration, Allocator> = self;
        me.view(view, sender);
    }
}