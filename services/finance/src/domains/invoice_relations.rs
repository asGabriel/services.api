use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct InvoiceRelations {
    pub parent_invoice_id: Uuid,
    pub child_invoice_id: Uuid
}