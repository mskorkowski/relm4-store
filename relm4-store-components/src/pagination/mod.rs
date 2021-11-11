//! Pagination component for store view

use record::DefaultIdAllocator;
use record::TemporaryIdAllocator;
use reexport::gtk;
use reexport::relm4;
use reexport::relm4_macros;
use store::DataStore;
use store::FactoryConfiguration;
use store::FactoryContainerWidgets;
use store::StoreViewImplementation;

use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::BoxExt;
use gtk::prelude::EntryExt;
use gtk::prelude::EntryBufferExtManual;
use gtk::prelude::ButtonExt;
use gtk::prelude::OrientableExt;
use gtk::prelude::WidgetExt;

use store::Pagination;
use store::StoreMsg;
use store::StoreView;

use relm4::ComponentUpdate;
use relm4::Model as ViewModel;
use relm4::send;
use relm4::Widgets;
use relm4::WidgetPlus;

use relm4_macros::widget;

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
}

/// Configuration of the pagination component
pub trait PaginationConfiguration<Allocator=DefaultIdAllocator> 
where Allocator: TemporaryIdAllocator,
{
    /// Type of parent view widgets
    /// 
    /// Type of ViewWidgets used by component which holds pagination component
    type ParentWidgets: FactoryContainerWidgets<Self::ParentViewModel, Allocator>;
    /// Type of parent view model
    /// 
    /// Type of model used by component which holds pagination component
    type ParentViewModel: ViewModel + FactoryConfiguration<Self::ParentWidgets, Allocator>;

    /// Returns a view which will be used by the pagination component
    fn get_view(parent_view_model: &Self::ParentViewModel) -> Rc<RefCell<StoreViewImplementation<Self::ParentWidgets, Self::ParentViewModel, Allocator>>>;
}

/// View model of the pagination component
#[derive(Debug)]
pub struct PaginationViewModel<Config, Allocator=DefaultIdAllocator> 
where 
    Config: PaginationConfiguration<Allocator> + 'static,
    Allocator: TemporaryIdAllocator + 'static,
{
    view: Rc<RefCell<StoreViewImplementation<Config::ParentWidgets, Config::ParentViewModel, Allocator>>>,
    page: gtk::EntryBuffer,
    pages_total: String,
}

impl<Config, Allocator> ViewModel for PaginationViewModel<Config, Allocator>
where 
    Config: PaginationConfiguration<Allocator>,
    Allocator: TemporaryIdAllocator,
{
    type Msg = PaginationMsg;
    type Widgets = PaginationWidgets;
    type Components = ();
}

impl<Config, Allocator> ComponentUpdate<Config::ParentViewModel> for PaginationViewModel<Config, Allocator>
where 
    Config: PaginationConfiguration<Allocator>,
    Allocator: TemporaryIdAllocator,
{
    fn init_model(parent_model: &Config::ParentViewModel) -> Self {
        let view = Config::get_view(parent_model); 


        let pages_total = format!("{}", view.borrow().total_pages());
        let current_page: &str = &format!("{}", view.borrow().current_page());
        Self{
            view: view.clone(),
            page: gtk::EntryBuffer::new(Some(current_page)),
            pages_total,
        }
    }

    fn update(
        &mut self, 
        msg: Self::Msg, 
        _components: &Self::Components, 
        _sender: relm4::Sender<Self::Msg>, 
        _parent_sender: relm4::Sender<<Config::ParentViewModel as ViewModel>::Msg>
    ) {
        match msg {
            PaginationMsg::First => 
                self.view.borrow().first_page(),
            PaginationMsg::Prev =>
                self.view.borrow().prev_page(),
            PaginationMsg::Next => 
                self.view.borrow().next_page(),
            PaginationMsg::Last => 
                self.view.borrow().last_page(),
            PaginationMsg::ToPage => (),
            PaginationMsg::Reload =>
                self.view.borrow().inbox(StoreMsg::Reload),
        }

        self.pages_total = self.view.borrow().total_pages().to_string();
        self.page.set_text(&self.view.borrow().current_page().to_string());

    }
}

/// Widgets for pagination component
#[widget(visibility=pub, relm4=relm4)]
impl<Config, Allocator> Widgets<PaginationViewModel<Config, Allocator>, Config::ParentViewModel> for PaginationWidgets 
where 
    Config: PaginationConfiguration<Allocator>,
    Allocator: TemporaryIdAllocator,
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
                // add_css_class: "flat",
            },
            append = &gtk::Label::with_mnemonic("/") {},
            append: max_page = &gtk::Label {
                set_label: &model.pages_total,
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