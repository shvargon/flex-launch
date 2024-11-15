use adw::glib::clone;
use adw::prelude::*;
use adw::subclass::prelude::*;
use ashpd::desktop::background::Background;
use ashpd::WindowIdentifier;
use gettextrs::gettext;
use gtk::{
    gio,
    glib::{self},
};

use crate::config::VERSION;
use crate::{FlexlaunchWindow, LaunchWindow};

mod imp {
    use super::*;
    use adw::gio::File;

    #[derive(Debug, Default)]
    pub struct FlexlaunchApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for FlexlaunchApplication {
        const NAME: &'static str = "FlexlaunchApplication";
        type Type = super::FlexlaunchApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for FlexlaunchApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
            obj.set_accels_for_action("app.background", &["<primary>b"]);
        }
    }

    impl ApplicationImpl for FlexlaunchApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            let application = self.obj();
            // Get the current window or create one if necessary
            let window = application.active_window().unwrap_or_else(|| {
                let window = FlexlaunchWindow::new(&*application);
                window.upcast()
            });
            self.request_background();

            window.present();
        }

        fn open(&self, files: &[File], _hint: &str) {
            for file in files {
                let application = self.obj();
                let window = LaunchWindow::new(&*application, file);
                // Ask the window manager/compositor to present the window
                window.present();
            }
        }
    }

    impl GtkApplicationImpl for FlexlaunchApplication {}
    impl AdwApplicationImpl for FlexlaunchApplication {}
}

glib::wrapper! {
    pub struct FlexlaunchApplication(ObjectSubclass<imp::FlexlaunchApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl FlexlaunchApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();

        let background_action = gio::ActionEntry::builder("background")
            .activate(move |app: &Self, _, _| app.request_background())
            .build();
        self.add_action_entries([quit_action, about_action, background_action]);
    }

    async fn port_request_background(&self) {
        if let Some(window) = self.active_window() {
            let root = window.native().unwrap();
            let identifier = WindowIdentifier::from_native(&root).await;
            let request = Background::request().identifier(identifier).reason(Some("work background"));

            match request.send().await.and_then(|r| r.response()) {
                Ok(response) => {
                    println!(" background set {:?}", response);
                }
                Err(err) => {
                    println!("error background set {:?}", err);
                }
            }
        }
    }

    fn request_background(&self) {
        println!("requesting background");
        let ctx = glib::MainContext::default();
        ctx.spawn_local(clone!(
            #[weak(rename_to = app)]
            self,
            async move { app.port_request_background().await }
        ));
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = adw::AboutDialog::builder()
            .application_name("flexlaunch")
            .application_icon("io.github.shvargon.flexlaunch")
            .developer_name("Valery Shvargonov")
            .version(VERSION)
            .developers(vec!["Valery Shvargonov"])
            // Translators: Replace "translator-credits" with your name/username, and optionally an email or URL.
            .translator_credits(&gettext("translator-credits"))
            .copyright("© 2024 Valery Shvargonov")
            .build();

        about.present(Some(&window));
    }
}
