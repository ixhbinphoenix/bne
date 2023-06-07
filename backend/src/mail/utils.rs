use lettre::{AsyncSmtpTransport, Tokio1Executor};
use tokio::{fs, io};

pub type Mailer = AsyncSmtpTransport<Tokio1Executor>;

pub async fn load_template(name: &str) -> io::Result<String> {
    let vec = fs::read(format!("email-templates/{}", name)).await?;
    Ok(String::from_utf8_lossy(&vec).into())
}
