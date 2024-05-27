use serde::Deserialize;

use crate::config::Config;

const SMTP_BAD_COMMAND: u16 = 500; // Syntax error, command unrecognized [This may include errors such as command line too long]
const SMTP_BAD_PARAM: u16 = 501; // Syntax error in parameters or arguments
const SMTP_UNKNOWN_COMMAND: u16 = 502; // Command not implemented
const SMTP_BAD_SEQUENCE: u16 = 503; // Bad sequence of commands
const SMTP_NOT_IMPLEMENTED: u16 = 504; // Command parameter not implemented

const SMTP_SYSTEM_STATUS: u16 = 211; // System status, or system help reply
const SMTP_HELP: u16 = 214; // Help message [Information on how to use the receiver or the meaning of a particular non-standard command; this reply is useful only to the human user]

const SMTP_SERVICE_READY: u16 = 220; // <host> Service ready
const SMTP_CLOSING_CHANNEL: u16 = 221; // <host> Service closing transmission channel
const SMTP_NOT_AVAILABLE: u16 = 421; // <host> Service not available, closing transmission channel [This may be a reply to any command if the service knows it must shut down]

const SMTP_OK: u16 = 250; // Requested mail action okay, completed
const SMTP_FORWARDING: u16 = 251; // User not local; will forward to <forward-path>
const SMTP_MAILBOX_BUSY: u16 = 450; // Requested mail action not taken: mailbox unavailable [E.g., mailbox busy]
const SMTP_MAILBOX_UNAVAILABLE: u16 = 550; // Requested action not taken: mailbox unavailable [E.g., mailbox not found, no access]
const SMTP_ERROR_IN_PROCESSING: u16 = 451; // Requested action aborted: error in processing
const SMTP_USER_NOT_LOCAL: u16 = 551; // User not local; please try <forward-path>
const SMTP_INSUFFICIENT_STORAGE: u16 = 452; // Requested action not taken: insufficient system storage
const SMTP_EXCEEDED_STORAGE_ALLOCATION: u16 = 552; // Requested mail action aborted: exceeded storage allocation
const SMTP_MAILBOX_NAME_NOT_ALLOWED: u16 = 553; // Requested action not taken: mailbox name not allowed [E.g., mailbox syntax incorrect]
const SMTP_START_MAIL_INPUT: u16 = 354; // Start mail input; end with <CRLF>.<CRLF>
const SMTP_TRANSACTION_FAILED: u16 = 554; // Transaction failed

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
#[repr(u16)]
pub(crate) enum SmtpReply {
    CommandSyntaxError = SMTP_BAD_COMMAND,
    ParamSyntaxError = SMTP_BAD_PARAM,
    UnknownCommand = SMTP_UNKNOWN_COMMAND,
    BadSequence = SMTP_BAD_SEQUENCE,
    NotImplemented = SMTP_NOT_IMPLEMENTED,
    SystemStatus = SMTP_SYSTEM_STATUS,
    Help = SMTP_HELP,
    ServiceReady = SMTP_SERVICE_READY,
    ClosingChannel = SMTP_CLOSING_CHANNEL,
    NotAvailable = SMTP_NOT_AVAILABLE,
    Ok = SMTP_OK,
    Forwarding = SMTP_FORWARDING,
    MailboxBusy = SMTP_MAILBOX_BUSY,
    MailboxUnavailable = SMTP_MAILBOX_UNAVAILABLE,
    ErrorInProcessing = SMTP_ERROR_IN_PROCESSING,
    UserNotLocal = SMTP_USER_NOT_LOCAL,
    InsufficientStorage = SMTP_INSUFFICIENT_STORAGE,
    ExceededStorage = SMTP_EXCEEDED_STORAGE_ALLOCATION,
    MailboxNameNotAllowed = SMTP_MAILBOX_NAME_NOT_ALLOWED,
    StartMailInput = SMTP_START_MAIL_INPUT,
    TransactionFailed = SMTP_TRANSACTION_FAILED,
}

impl SmtpReply {
    fn default_reply_message(&self) -> String {
        match self {
            SmtpReply::ServiceReady => "localhost ESMTP dmail".into(), //todo fix hostname

            _ => todo!("Message for {} is unimplemented", *self as u16),
        }
    }

    fn get_reply_message(&self) -> String {
        let default = self.default_reply_message();
        format!(
            "{} {}",
            *self as u16,
            match Config::load().unwrap().smtp.reply_messages.get(self) {
                Some(msg) => msg,
                None => &default,
            }
        )
    }
}

impl std::fmt::Display for SmtpReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_reply_message())
    }
}
