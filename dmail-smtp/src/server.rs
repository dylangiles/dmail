use crate::smtp::SmtpReply;

pub(crate) struct SmtpSession {}

impl SmtpSession {
    pub(crate) const fn new() -> Self {
        Self {}
    }
    pub(crate) const fn service_ready(&self) -> SmtpReply {
        SmtpReply::ServiceReady
    }
}
