use reexport::relm4;

use relm4::ComponentUpdate;
use relm4::Model as ViewModel;

use store::DataStoreBase;
use store::StoreMsg;

use super::InMemoryBackend;
use super::InMemoryBackendBuilder;

impl<Builder> ViewModel for InMemoryBackend<Builder> 
where Builder: InMemoryBackendBuilder
{
    type Msg = StoreMsg<Builder::DataModel>;
    type Widgets = ();
    type Components = ();
}

impl<Builder, ParentViewModel> ComponentUpdate<ParentViewModel> for InMemoryBackend<Builder>
where 
    Builder: InMemoryBackendBuilder,
    ParentViewModel: ViewModel,
{
    fn init_model(_parent_model: &ParentViewModel) -> Self {
        Self::new(Builder::initial_data())
    }

    fn update(
        &mut self, 
        msg: Self::Msg, 
        _components: &Self::Components, 
        _sender: relm4::Sender<Self::Msg>, 
        _parent_sender: relm4::Sender<ParentViewModel::Msg>
    ) {
       self.inbox(msg) 
    }
}