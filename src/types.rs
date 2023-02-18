use crate::models::NewOrder;
use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

/// Orders can be only Buy or Sell
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Buy,
    Sell,
}

impl FromStr for Kind {
    type Err = ();

    fn from_str(kind: &str) -> std::result::Result<Self, Self::Err> {
        match kind {
            "Buy" => std::result::Result::Ok(Self::Buy),
            "Sell" => std::result::Result::Ok(Self::Sell),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Each status that an order can have
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Status {
    Active,
    Canceled,
    CanceledByAdmin,
    CompletedByAdmin,
    Dispute,
    Expired,
    FiatSent,
    SettledHoldInvoice,
    Pending,
    Success,
    WaitingBuyerInvoice,
    WaitingPayment,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Active" => std::result::Result::Ok(Self::Active),
            "Canceled" => std::result::Result::Ok(Self::Active),
            "CanceledByAdmin" => std::result::Result::Ok(Self::Active),
            "CompletedByAdmin" => std::result::Result::Ok(Self::Active),
            "Dispute" => std::result::Result::Ok(Self::Active),
            "Expired" => std::result::Result::Ok(Self::Active),
            "FiatSent" => std::result::Result::Ok(Self::Active),
            "SettledHoldInvoice" => std::result::Result::Ok(Self::Active),
            "Pending" => std::result::Result::Ok(Self::Active),
            "Success" => std::result::Result::Ok(Self::Active),
            "WaitingBuyerInvoice" => std::result::Result::Ok(Self::Active),
            "WaitingPayment" => std::result::Result::Ok(Self::Active),
            _ => Err(()),
        }
    }
}

/// Action is used to identify each message between Mostro and users
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Action {
    Order,
    TakeSell,
    TakeBuy,
    PayInvoice,
    FiatSent,
    Release,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Use this Message to establish communication between users and Mostro
#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub version: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<Uuid>,
    pub action: Action,
    pub content: Option<Content>,
}

/// Message content
#[derive(Debug, Deserialize, Serialize)]
pub enum Content {
    Order(NewOrder),
    PaymentRequest(String),
}

#[allow(dead_code)]
impl Message {
    /// New message from json string
    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }
    /// Get message as json string
    pub fn as_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }

    /// Verify if is valid message
    pub fn verify(&self) -> bool {
        match &self.action {
            Action::Order => matches!(&self.content, Some(Content::Order(_))),
            Action::TakeSell => {
                if self.order_id.is_none() {
                    return false;
                }
                matches!(&self.content, Some(Content::PaymentRequest(_)))
            }
            Action::TakeBuy => {
                if self.order_id.is_none() {
                    return false;
                }
                true
            }
            Action::FiatSent => {
                if self.order_id.is_none() {
                    return false;
                }
                true
            }
            Action::Release => {
                if self.order_id.is_none() {
                    return false;
                }
                true
            }
            _ => false,
        }
    }

    pub fn get_order(&self) -> Option<&NewOrder> {
        if self.action != Action::Order {
            return None;
        }
        match &self.content {
            Some(Content::Order(o)) => Some(o),
            _ => None,
        }
    }

    pub fn get_payment_request(&self) -> Option<String> {
        if self.action != Action::TakeSell {
            return None;
        }
        match &self.content {
            Some(Content::PaymentRequest(pr)) => Some(pr.to_owned()),
            _ => None,
        }
    }
}
