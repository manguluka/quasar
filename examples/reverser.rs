extern crate quasar;
extern crate mustache;
extern crate rustc_serialize;

use quasar::{compile_str, Component, EventType};

#[derive(RustcEncodable)]
struct ReverseData {
    message: String,
}


fn main() {
    let app = quasar::init();

    let my_widget = Component {
        data: ReverseData {
            message: "Hello World".to_owned()
        },
        props: vec!["something"],
        // methods: vec![leftpad, reverse, slugify],
        template: compile_str(r##"
            <p>{{ props.something }}, {{ message }}</p>
            <button>Reverse Message</button>
        "##).expect("failed to compile my_widget template")
    };

    let views = app.bind(my_widget, "Reverser");
    views.on(EventType::Click, |mut evt| {
        println!("on click called");
        let mut data = evt.view.data_mut();
        data.message = data.message.chars().rev().collect::<String>();
    });

    println!("End of main");
    app.spin();
}