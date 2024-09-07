pub mod excel2json;
pub mod mailer;
pub mod utils;
pub mod xlsx;

use handlebars::Handlebars;
use utils::*;

fn main() {
    let mut mailer = mailer::Mailer::from_json("data.json");
    let reg = Handlebars::new();
    let cfg = read_json("data.json");

    for data in xlsx::parse("receivers.xlsx") {
        let template_string = read_file("message.txt");

        let parsed = reg
            .render_template(&template_string, &data)
            .expect("Fail on handlebars parser");

        let name = data[cfg["xlsx"]["name"]
            .as_str()
            .expect("error getting name from json")]
        .as_str()
        .expect("error getting name");

        let mail = data[cfg["xlsx"]["mail"]
            .as_str()
            .expect("error getting mail from json")]
        .as_str()
        .expect("error getting mail");

        let subj = parsed.split('\n').collect::<Vec<&str>>()[0];

        mailer
            .set_subject(subj)
            .set_content(&parsed)
            .send_email(name, mail);
    }
}
