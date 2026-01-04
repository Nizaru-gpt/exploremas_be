use lettre::{
    message::{header::ContentType, Mailbox, Message},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
};

pub async fn send_otp_email(to_email: &str, otp: &str) -> Result<(), String> {
    let smtp_host = std::env::var("SMTP_HOST").map_err(|_| "SMTP_HOST not set".to_string())?;
    let smtp_port: u16 = std::env::var("SMTP_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(587);

    let smtp_user = std::env::var("SMTP_USER").map_err(|_| "SMTP_USER not set".to_string())?;
    let smtp_pass = std::env::var("SMTP_PASS").map_err(|_| "SMTP_PASS not set".to_string())?;

    // FROM: harus email yang sama dengan SMTP_USER biar Gmail gak curiga
    let from_mailbox: Mailbox = smtp_user
        .parse()
        .map_err(|e| format!("Invalid SMTP_USER email: {e:?}"))?;

    let to_mailbox: Mailbox = to_email
        .parse()
        .map_err(|e| format!("Invalid to email: {e:?}"))?;

    let subject = "OTP Reset Password - ExploreMas";
    let body = format!(
        "Kode OTP kamu: {otp}\n\nJika kamu tidak meminta reset password, abaikan email ini."
    );

    let email = Message::builder()
        .from(from_mailbox)
        .to(to_mailbox)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .map_err(|e| format!("Build email error: {e:?}"))?;

    let creds = Credentials::new(smtp_user, smtp_pass);

    // STARTTLS port 587
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp_host)
        .map_err(|e| format!("SMTP starttls_relay error: {e:?}"))?
        .port(smtp_port)
        .credentials(creds)
        .build();

    mailer
        .send(email)
        .await
        .map_err(|e| format!("SMTP send error: {e:?}"))?;

    Ok(())
}
