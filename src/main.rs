use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::Email;
//use std::path::Path;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let dat1 = &args[1];
    let dat2 = &args[2];

    println!("Dat1 {}", dat1);
    println!("Dat2 {}", dat2);
    let resultt = [dat1, "\n", dat2].join("\n");
    let email = Email::builder()
        .to("buendiaenrb@gmail.com")
        .from("buendiaenrb@gmail.com")
        .subject("encuesta e-learning")
        //.html(result)
        .text(resultt)
        //.attachment_from_file(Path::new("cita.pdf"), None, &mime::APPLICATION_PDF)
        //.unwrap()
        .build()
        .unwrap();

    let creds = Credentials::new(
        "buendiaenrb@gmail.com".to_string(),
        "patopatosuperman".to_string(),
    );

    // Open connection to gmail
    let mut mailer = SmtpClient::new_simple("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .transport();

    // Send the email
    let result = mailer.send(email.into());

    if result.is_ok() {
        println!("Enviado ...");
    } else {
        println!("No se puede enviar: {:?}", result);
    }

    //assert!(result.is_ok());
}
