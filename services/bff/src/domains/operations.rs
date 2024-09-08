use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::Month;
use finance_domains::{entries::{Entry, EntryStatus}, invoices::Invoice};
use uuid::Uuid;

pub struct OperationsPage {
    total: i16,
    operations: Vec<InvoiceWithEntriesDetails>,
}

pub struct InvoiceWithEntriesDetails {
    invoice_id: Uuid,
    title: String,
    month: Month,
    year: i16,
    entries_summary: EntriesSummaryDetails,
}

pub struct EntriesSummaryDetails {
    revenue: BigDecimal,
    expenses: BigDecimal,
    total_entries: u16,
}

impl InvoiceWithEntriesDetails {
    pub fn build(invoice: Invoice, entries: Vec<Entry>) -> Self {
        let total_revenue = entries.iter().fold(BigDecimal::from_f64(0.0).unwrap(), |acc, entry| {
            if entry.status == EntryStatus::Completed {
                acc + entry.get_value()
            } else {
                acc
            }
        });
    }

    todo!()
}
