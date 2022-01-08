//! Pagination component for store view

use reexport::gtk;
use reexport::relm4;
use reexport::relm4_macros;
use reexport::tracker;

use gtk::prelude::BoxExt;
use gtk::prelude::EntryExt;
use gtk::prelude::EntryBufferExtManual;
use gtk::prelude::ButtonExt;
use gtk::prelude::OrientableExt;
use gtk::prelude::WidgetExt;


use relm4::ComponentUpdate;
use relm4::Model as ViewModel;
use relm4::send;
use relm4::Widgets;
use relm4::WidgetPlus;

use relm4_macros::widget;

use store::StoreView;
use store::DataStore;
use store::StoreViewMsg;
use store::StoreViewPrototype;
use store::Pagination;
use store_view::View;

/// Messages sent to pagination component
#[derive(Debug)]
pub enum PaginationMsg {
    /// Go to first page
    First,
    /// Go to last page
    Last,
    /// Go to next page
    Next,
    /// Go to previous page
    Prev,
    /// Go to page
    ToPage,
    /// Reload store
    Reload,
    /// Event to be sent when store was updated
    StoreUpdated,
}

/// Configuration of the pagination component
pub trait PaginationConfiguration
{
    /// Type of parent view model
    /// 
    /// Type of model used by component which holds pagination component
    type StoreViewPrototype: StoreViewPrototype;

    /// Returns a view which will be used by the pagination component
    fn get_view(parent_view_model: &<Self::StoreViewPrototype as StoreViewPrototype>::ViewModel) 
        -> View<Self::StoreViewPrototype>;
}

/// View model of the pagination component
#[tracker::track]
#[derive(Debug)]
pub struct PaginationViewModel<Config>
where
    Config: PaginationConfiguration + 'static,
{
    #[do_not_track]
    view: View<Config::StoreViewPrototype>,
    #[do_not_track]
    page: gtk::EntryBuffer,
    total_pages: String,
}

impl<Config> ViewModel for PaginationViewModel<Config>
where 
    Config: PaginationConfiguration,
{
    type Msg = PaginationMsg;
    type Widgets = PaginationWidgets;
    type Components = ();
}

impl<Config> ComponentUpdate<<Config::StoreViewPrototype as StoreViewPrototype>::ViewModel> for PaginationViewModel<Config>
where 
    Config: PaginationConfiguration,
{
    fn init_model(parent_model: &<Config::StoreViewPrototype as StoreViewPrototype>::ViewModel) -> Self {
        let view = Config::get_view(parent_model); 

        let total_pages = format!("{}", view.total_pages());
        let current_page: &str = &format!("{}", view.current_page());

        Self{
            view: view.clone(),
            page: gtk::EntryBuffer::new(Some(current_page)),
            total_pages,
            tracker: 0,
        }
    }

    fn update(
        &mut self, 
        msg: Self::Msg, 
        _components: &Self::Components, 
        _sender: relm4::Sender<Self::Msg>, 
        _parent_sender: relm4::Sender<<<Config::StoreViewPrototype as StoreViewPrototype>::ViewModel as ViewModel>::Msg>
    ) {
        match msg {
            PaginationMsg::First => 
                self.view.first_page(),
            PaginationMsg::Prev =>
                self.view.prev_page(),
            PaginationMsg::Next => 
                self.view.next_page(),
            PaginationMsg::Last => 
                self.view.last_page(),
            PaginationMsg::ToPage => (),
            PaginationMsg::Reload =>
                self.view.send(StoreViewMsg::Reload),
            PaginationMsg::StoreUpdated => (),
        }

        let new_total_pages = self.view.total_pages().to_string(); 
        self.set_total_pages(new_total_pages);
        self.page.set_text(&self.view.current_page().to_string());
    }
}

/// Widgets for pagination component
#[widget(visibility=pub, relm4=relm4)]
impl<Config> Widgets<PaginationViewModel<Config>, <Config::StoreViewPrototype as StoreViewPrototype>::ViewModel> for PaginationWidgets 
where 
    Config: PaginationConfiguration,
{
    view! {
        root = &gtk::Box {
            set_margin_all: 12,
            set_orientation: gtk::Orientation::Horizontal,
            append: first = &gtk::Button::from_icon_name(Some("go-first-symbolic")) {
                connect_clicked(sender) => move |_| {
                    send!(sender, PaginationMsg::First);
                },
                add_css_class: "flat",
            },
            append: prev = &gtk::Button::from_icon_name(Some("go-previous-symbolic")) {
                connect_clicked(sender) => move |_| {
                    send!(sender, PaginationMsg::Prev)
                },
                add_css_class: "flat",
            },
            append: page = &gtk::Entry::with_buffer(&model.page) {
                connect_activate(sender) => move |_| {
                    send!(sender, PaginationMsg::ToPage)
                },
            },
            append = &gtk::Label::with_mnemonic("/") {},
            append: max_page = &gtk::Label {
                set_text: track!(
                    model.changed(PaginationViewModel::<Config>::total_pages()),
                    &model.total_pages
                ),
            },
            append: reload = &gtk::Button::from_icon_name(Some("view-refresh-symbolic")) {
                connect_clicked(sender) => move |_| {
                    send!(sender, PaginationMsg::Reload)
                },
                add_css_class: "flat",
            },
            append: next = &gtk::Button::from_icon_name(Some("go-next-symbolic")) {
                connect_clicked(sender) => move |_| {
                    send!(sender, PaginationMsg::Next)
                },
                add_css_class: "flat",
            },
            append: last = &gtk::Button::from_icon_name(Some("go-last-symbolic")) {
                connect_clicked(sender) => move |_| {
                    send!(sender, PaginationMsg::Last)
                },
                add_css_class: "flat",
            },
        }
    }
}