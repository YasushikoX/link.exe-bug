use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn send_mail(){
    let email = Message::builder()
        .from(Mailbox::new(Some("NoBody".to_owned()), "sparkle98989@gmail.com".parse().unwrap()))
        .to(Mailbox::new(Some("Jake".to_owned()), "dimov.jake@gmail.com".parse().unwrap()))
        .subject("Happy new year")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from("Be happy!"))
        .unwrap();

    let creds = Credentials::new("********".to_owned(), "********".to_owned());

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();


    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }

    println!("It maybe was sent, god knows lol")
}