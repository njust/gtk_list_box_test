#[macro_use]
extern crate glib;
extern crate gio;
extern crate gtk;

use glib_data_model_helper::data_model;

use glib_data_model_helper::prelude::*;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;


fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("ListBox Model Sample");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(320, 480);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    let model = gio::ListStore::new(GerdaData::static_type());

    let listbox = gtk::ListBox::new();
    listbox.bind_model(Some(&model),
                       clone!(@weak window => @default-panic, move |item| {
        let row = gtk::ListBoxRow::new();
        let item = item.downcast_ref::<GerdaData>().expect("Row data is of wrong type");
        let label = gtk::Label::new(None);
        item.bind_property("name", &label, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();

        row.add(&label);
        row.show_all();
        row.upcast::<gtk::Widget>()
    }));

    let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scrolled_window.add(&listbox);

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    let add_button = gtk::Button::with_label("Add");
    add_button.connect_clicked(clone!(@weak model => move |_| {
        model.append(&GerdaData::new(&[("name", &"hudel"), ("count", &42)]));
    }));

    hbox.add(&add_button);

    let delete_button = gtk::Button::with_label("Delete");
    delete_button.connect_clicked(clone!(@weak model, @weak listbox => move |_| {
        let selected = listbox.get_selected_row();
        if let Some(selected) = selected {
            let idx = selected.get_index();
            model.remove(idx as u32);
        }
    }));
    hbox.add(&delete_button);

    vbox.pack_start(&hbox, false, false, 0);
    vbox.pack_start(&scrolled_window, true, true, 0);

    window.add(&vbox);

    for i in 0..10 {
        model.append(&GerdaData::new(&[("name", &format!("name: {}", i)), ("count", &i)]));
    }

    window.show_all();
}

data_model!(GerdaData);
impl DataModelDescription for GerdaData {
    const NAME: &'static str = "GerdaData";
    fn get_properties() -> &'static [Property<'static>] {
        &[
            subclass::Property("name", |name| {
                glib::ParamSpec::string(name,"Name","Name",None, glib::ParamFlags::READWRITE)
            }),
            subclass::Property("count", |name| {
                glib::ParamSpec::int(name,"Count","Count",0,100,0, glib::ParamFlags::READWRITE)
            }),
        ]
    }
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.listbox-model"),
        Default::default(),
    )
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}