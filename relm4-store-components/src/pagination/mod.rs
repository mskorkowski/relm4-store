//! Pagination component for store view

use reexport::gtk;
use reexport::relm4;
use reexport::relm4_macros;
use reexport::tracker;

use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::BoxExt;
use gtk::prelude::EntryExt;
use gtk::prelude::EntryBufferExtManual;
use gtk::prelude::ButtonExt;
use gtk::prelude::OrientableExt;
use gtk::prelude::WidgetExt;

use record::DefaultIdAllocator;
use record::TemporaryIdAllocator;
use store::DataStore;
use store::FactoryConfiguration;
use store::FactoryContainerWidgets;
use store::Pagination;
use store::StoreMsg;
use store::StoreView;
use store::StoreViewImplementation;
use store::StoreViewInnerComponent;

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
    /// Event to be sent when store was updated
    StoreUpdated,
}

/// Configuration of the pagination component
pub trait PaginationConfiguration<Allocator=DefaultIdAllocator> 
where 
    Allocator: TemporaryIdAllocator,
    <<Self::FactoryConfiguration as FactoryConfiguration<Allocator>>::ViewModel as ViewModel>::Widgets: 
        relm4::Widgets<<Self::FactoryConfiguration as FactoryConfiguration<Allocator>>::ViewModel, <Self::FactoryConfiguration as FactoryConfiguration<Allocator>>::ParentViewModel> +
        FactoryContainerWidgets<Self::FactoryConfiguration, Allocator>,
    <<Self::FactoryConfiguration as FactoryConfiguration<Allocator>>::ViewModel as ViewModel>::Components: 
        StoreViewInnerComponent<<Self::FactoryConfiguration as FactoryConfiguration<Allocator>>::ViewModel>,
{
    /// Type of parent view model
    /// 
    /// Type of model used by component which holds pagination component
    type FactoryConfiguration: FactoryConfiguration<Allocator>;

    /// Returns a view which will be used by the pagination component
    fn get_view(parent_view_model: &<Self::FactoryConfiguration as FactoryConfiguration<Allocator>>::ViewModel) 
        -> Rc<RefCell<StoreViewImplementation<Self::FactoryConfiguration, Allocator>>>;
}

/// View model of the pagination component
#[tracker::track]
#[derive(Debug)]
pub struct PaginationViewModel<Config: PaginationConfiguration<Allocator> + 'static, Allocator: TemporaryIdAllocator + 'static =DefaultIdAllocator>
{
    #[do_not_track]
    view: Rc<RefCell<StoreViewImplementation<Config::FactoryConfiguration, Allocator>>>,
    #[do_not_track]
    page: gtk::EntryBuffer,
    total_pages: String,
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

impl<Config, Allocator> ComponentUpdate<<Config::FactoryConfiguration as FactoryConfiguration<Allocator>>::ViewModel> for PaginationViewModel<Config, Allocator>
where 
    Config: PaginationConfiguration<Allocator>,
    Allocator: TemporaryIdAllocator,
    <<Config::FactoryConfiguration as FactoryConfiguration<Allocator>>::ViewModel as ViewModel>::Widgets: 
        relm4::Widgets<<Config::FactoryConfiguration as FactoryConfiguration<Allocator>>::ViewModel, <Config::FactoryConfiguration as FactoryConfiguration<Allocator>>::ParentViewModel> +
        FactoryContainerWidgets<Config::FactoryConfiguration, Allocator>,
    <<Config::FactoryConfiguration as FactoryConfiguration<Allocator>>::ViewModel as ViewModel>::Components: 
        StoreViewInnerComponent<<Config::FactoryConfiguration as FactoryConfiguration<Allocator>>::ViewModel>,
{
    fn init_model(parent_model: &<Config::FactoryConfiguration as FactoryConfiguration<Allocator>>::ViewModel) -> Self {
        let view = Config::get_view(parent_model); 

        let total_pages = format!("{}", view.borrow().total_pages());
        let current_page: &str = &format!("{}", view.borrow().current_page());

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
        _parent_sender: relm4::Sender<<<Config::FactoryConfiguration as FactoryConfiguration<Allocator>>::ViewModel as ViewModel>::Msg>
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
            PaginationMsg::StoreUpdated => (),
        }

        let new_total_pages = self.view.borrow().total_pages().to_string(); 
        self.set_total_pages(new_total_pages);
        self.page.set_text(&self.view.borrow().current_page().to_string());
    }
}

/// Widgets for pagination component
#[widget(visibility=pub, relm4=relm4)]
impl<Config, Allocator> Widgets<PaginationViewModel<Config, Allocator>, <Config::FactoryConfiguration as FactoryConfiguration<Allocator>>::ViewModel> for PaginationWidgets 
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
            },
            append = &gtk::Label::with_mnemonic("/") {},
            append: max_page = &gtk::Label {
                set_text: track!(
                    model.changed(PaginationViewModel::<Config, Allocator>::total_pages()),
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