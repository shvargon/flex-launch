use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/shvargon/flexlaunch/window.ui")]
    pub struct FlexlaunchWindow {
        // Template widgets
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FlexlaunchWindow {
        const NAME: &'static str = "FlexlaunchWindow";
        type Type = super::FlexlaunchWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for FlexlaunchWindow {}
    impl WidgetImpl for FlexlaunchWindow {}
    impl WindowImpl for FlexlaunchWindow {}
    impl ApplicationWindowImpl for FlexlaunchWindow {}
    impl AdwApplicationWindowImpl for FlexlaunchWindow {}
}

glib::wrapper! {
    pub struct FlexlaunchWindow(ObjectSubclass<imp::FlexlaunchWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl FlexlaunchWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
