use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib, FlowBox};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/shvargon/flexlaunch/launch-window.ui")]
    pub struct LaunchWindow {
        #[template_child]
        pub applist: TemplateChild<FlowBox>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LaunchWindow {
        const NAME: &'static str = "LaunchWindow";
        type Type = super::LaunchWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for LaunchWindow {}
    impl WidgetImpl for LaunchWindow {}
    impl WindowImpl for LaunchWindow {}
    impl ApplicationWindowImpl for LaunchWindow {}
    impl AdwApplicationWindowImpl for LaunchWindow {}
}

glib::wrapper! {
    pub struct LaunchWindow(ObjectSubclass<imp::LaunchWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl LaunchWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P, file: &gio::File) -> Self {
        dbg!(file);
        let window: LaunchWindow = glib::Object::builder()
            .property("application", application)
            .build();

        let applist = window.imp().applist.get();

        // Пример: добавление кнопок в FlowBox
        for i in 0..10 {
            let button = gtk::Button::builder()
                .label(&format!("Item {}", i + 1))
                .build();
            applist.append(&button);
        }

        window
    }
}
