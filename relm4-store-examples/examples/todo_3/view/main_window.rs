use reexport::{gtk, relm4, relm4_macros};
use gtk::prelude::{BoxExt, ButtonExt, OrientableExt, GtkWindowExt};
use relm4::{AppUpdate, Components, Model as ViewModel, send, Sender, Widgets};
use relm4_macros::widget;
use store::{OrderedStore, StoreSize, StoreViewComponent, window::PositionTrackingWindow};

use crate::{
    store::{Tasks, OrderTasksBy},
    view::{task_list::TasksListConfiguration, task_list::TasksListViewModel}
};

pub enum MainWindowMsg {
    ASC,
    DESC,
}

pub struct MainWindowViewModel {
    pub tasks: Tasks,
    pub page_size: usize,
}

impl ViewModel for MainWindowViewModel {
    type Msg = MainWindowMsg;
    type Widgets = MainWindowWidgets;
    type Components = MainWindowComponents;
}

impl AppUpdate for MainWindowViewModel {
    fn update(
        &mut self, 
        msg: Self::Msg, 
        _components: &Self::Components, 
        _sender: Sender<Self::Msg>
    ) -> bool {

        match msg  {
            MainWindowMsg::ASC => {
                self.tasks.set_order(
                    OrderTasksBy::Name{ascending: true}
                )
            },
            MainWindowMsg::DESC => {
                self.tasks.set_order(
                    OrderTasksBy::Name{ascending: false}
                )
            }
        }

        true
    }
}

pub struct MainWindowComponents {
    tasks_list_1: StoreViewComponent<TasksListViewModel<TaskList1Configuration>>,
}

impl Components<MainWindowViewModel> for MainWindowComponents {
    fn init_components(
        parent_view_model: &MainWindowViewModel,
        _parent_sender: Sender<MainWindowMsg>,
    ) -> Self {
        Self {
            tasks_list_1: StoreViewComponent::new(parent_view_model, parent_view_model.tasks.clone(), StoreSize::Items(parent_view_model.page_size)),
        }
    }

    fn connect_parent(&mut self, _parent_widgets: &MainWindowWidgets) {}
}

struct TaskList1Configuration {}
impl TasksListConfiguration for TaskList1Configuration {
    type ParentViewModel = MainWindowViewModel;
    type Window = PositionTrackingWindow;
    fn get_tasks(parent_model: &Self::ParentViewModel) -> Tasks {
        parent_model.tasks.clone()
    }
}

#[widget(visibility=pub, relm4=relm4)]
impl Widgets<MainWindowViewModel, ()> for MainWindowWidgets {
    view!{
        root = gtk::ApplicationWindow {
            set_child= Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    append = &gtk::Button::with_label("ASC") {
                        connect_clicked(sender) => move |_| {
                            send!(sender, MainWindowMsg::ASC)
                        }
                    },
                    append = &gtk::Button::with_label("DESC") {
                        connect_clicked(sender) => move |_| {
                            send!(sender, MainWindowMsg::DESC)
                        }
                    }
                },
                append: components.tasks_list_1.root_widget(),
            },
            set_default_size: args!(300, 600),
        }
    }
}