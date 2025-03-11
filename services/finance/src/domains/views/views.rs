use serde::{Deserialize, Serialize};

use crate::domains::{entries::Entry, invoices::Invoice};

pub type InvoiceDetailsView = InvoiceDetailsViewResponse;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceDetailsViewResponse {
    pub main_invoice: InvoiceDetails,
    pub sub_invoices: Vec<InvoiceDetails>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceDetails {
    #[serde(flatten)]
    pub invoice: Invoice,
    pub entries: Vec<Entry>
}