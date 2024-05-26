use lettre::{AsyncSmtpTransport, Tokio1Executor};
use tokio::io;

pub type Mailer = AsyncSmtpTransport<Tokio1Executor>;

pub async fn load_template(name: &str) -> io::Result<String> {
    // let vec = fs::read(format!("email-templates/{}", name)).await?;
    let string = match name {
        "email_change.html" => include_str!("../../email-templates/email_change.html"),
        "email_changed.html" => include_str!("../../email-templates/email_changed.html"),
        "gdpr_compliance.html" => include_str!("../../email-templates/gdpr_compliance.html"),
        "password_changed.html" => include_str!("../../email-templates/password_changed.html"),
        "password_reset.html" => include_str!("../../email-templates/password_reset.html"),
        "verify.html" => include_str!("../../email-templates/verify.html"),
        _ => {
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("Template {} not found", name)))
        }
    };
    Ok(string.to_string())
}
