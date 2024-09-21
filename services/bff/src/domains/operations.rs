use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::Month;
use finance_domains::{
    entries::{Entry, EntryStatus, EntryType},
    invoices::Invoice,
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationsPage {
    pub total: usize,
    pub operations: Vec<InvoiceWithEntriesDetails>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceDetailsPage {
    pub invoice: InvoiceWithEntriesDetails,
    pub entries: Vec<Entry>
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceWithEntriesDetails {
    invoice_id: Uuid,
    title: String,
    month: Month,
    year: i16,
    entries_summary: EntriesSummaryDetails,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntriesSummaryDetails {
    revenue: BigDecimal,
    expenses: BigDecimal,
    total_entries: usize,
}

impl InvoiceWithEntriesDetails {
    pub fn build(invoice: &Invoice, entries: Vec<&Entry>) -> Self {
        let total_revenue =
            entries
                .iter()
                .fold(BigDecimal::from_f64(0.0).unwrap(), |acc, entry| {
                    if entry.status == EntryStatus::Completed
                        && entry.entry_type == EntryType::Revenue
                    {
                        acc + entry.value.normalized()
                    } else {
                        acc
                    }
                });

        let total_expenses =
            entries
                .iter()
                .fold(BigDecimal::from_f64(0.0).unwrap(), |acc, entry| {
                    if entry.status == EntryStatus::Completed
                        && entry.entry_type == EntryType::Expense
                    {
                        acc + entry.value.normalized()
                    } else {
                        acc
                    }
                });

        InvoiceWithEntriesDetails {
            invoice_id: invoice.invoice_id,
            title: String::from(&invoice.title),
            month: Month::from_i32(invoice.month).unwrap(),
            year: invoice.year,
            entries_summary: EntriesSummaryDetails {
                expenses: total_expenses,
                revenue: total_revenue,
                total_entries: entries.len(),
            },
        }
    }
}
