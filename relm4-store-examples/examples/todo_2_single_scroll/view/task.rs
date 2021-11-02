use reexport::{gtk, relm4};
use gtk::{Box, CheckButton, Label, Orientation, 
    prelude::{BoxExt, CheckButtonExt}};
use relm4::{send, Sender};
use model::{Id, Identifiable};
use store::{FactoryBuilder, Position, window::PositionTrackingWindow};
use crate::model::Task;
use crate::store::Tasks;

pub enum TaskMsg {
    Toggle{
        complete: bool,
        id: Id<Task>,
    },
    New,
    Scrolled,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct TaskWidgets {
    checkbox: CheckButton,
    label: Label,
    root: Box,
}

pub struct TaskFactoryBuilder {}

impl FactoryBuilder for TaskFactoryBuilder {
    type Store = Tasks;
    type Widgets = TaskWidgets;
    type Root = gtk::Box;
    type View = gtk::Box;
    type Window = PositionTrackingWindow;
    type Msg = TaskMsg;

    fn generate(
        record: &Task,
        _position: Position,
        sender: Sender<TaskMsg>,
    ) -> Self::Widgets {
        let margin_size = 0;

        let root = Box::builder()
            .orientation(Orientation::Horizontal)
            .build();

        let checkbox = CheckButton::builder()
            .margin_top(margin_size)
            .margin_start(margin_size)
            .margin_end(margin_size)
            .margin_bottom(margin_size)
            .active(record.completed)
            .build();

        {
            let sender = sender.clone();
            let id = record.get_id();

            checkbox.connect_toggled(move |btn| {
                send!(sender, TaskMsg::Toggle{
                    id,
                    complete: btn.is_active()
                });
            });
        }

        let label = Label::builder()
            .margin_top(margin_size)
            .margin_start(margin_size)
            .margin_end(margin_size)
            .margin_bottom(margin_size)
            .label(&record.description)
            .build();

        root.append(&checkbox);
        root.append(&label);

        TaskWidgets {
            checkbox,
            label,
            root,
        }
    }

    /// Function called when record is modified.
    fn update(
        record: Task,
        _position: Position,
        widgets: &Self::Widgets,
    ) {
        widgets.checkbox.set_active(record.completed);

        let attrs = widgets.label.attributes().unwrap_or_default();
        attrs.change(gtk::pango::Attribute::new_strikethrough(record.completed));
        widgets.label.set_attributes(Some(&attrs));
    }

    fn position(
        _model: Task, 
        _position: Position,
    ) {}

    /// Get the outermost widget from the widgets.
    fn get_root(widgets: &Self::Widgets) -> &Self::Root {
        &widgets.root
    }
}
