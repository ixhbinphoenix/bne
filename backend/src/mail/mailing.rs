use lettre::{Message, message::IntoBody, AsyncTransport};

use crate::prelude::Error;

use super::{error::MailError, utils::Mailer};

pub fn build_mail<T>(to: &str, subject: &str, body: T) -> Result<Message, Error>
    where T: IntoBody
{
    Message::builder()
        .from("TheSchedule <noreply@theschedule.de>".parse().unwrap())
        .to(to.parse().map_err(|_| Error::Mail(MailError::InvalidAddress(to.into())))?)
        .subject(subject)
        .body(body)
        .map_err(|e| MailError::MessageCreation(e).into())
}

pub async fn send_mail(mailer: actix_web::web::Data<Mailer>, message: Message) -> Result<(), Error> {
    mailer.send(message).await
        .map_err(|e| Error::Mail(MailError::SMTP(e)))
        .map(|_| ())
}
