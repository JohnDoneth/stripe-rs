// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::{CustomerId, InvoiceId, InvoiceItemId, SubscriptionId};
use crate::params::{Deleted, Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{Currency, Customer, Invoice, Period, Plan, Subscription, TaxRate};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "InvoiceItem".
///
/// For more details see [https://stripe.com/docs/api/invoiceitems/object](https://stripe.com/docs/api/invoiceitems/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvoiceItem {
    /// Unique identifier for the object.
    pub id: InvoiceItemId,

    /// Amount (in the `currency` specified) of the invoice item.
    ///
    /// This should always be equal to `unit_amount * quantity`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// The ID of the customer who will be billed when this invoice item is billed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Timestamp>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// If true, discounts will apply to this invoice item.
    ///
    /// Always false for prorations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,

    /// The ID of the invoice this invoice item belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Expandable<Invoice>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,

    /// If the invoice item is a proration, the plan of the subscription that the proration was computed for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,

    /// Whether the invoice item was created automatically as a proration adjustment when the customer switched plans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration: Option<bool>,

    /// Quantity of units for the invoice item.
    ///
    /// If the invoice item is a proration, the quantity of the subscription that the proration was computed for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The subscription that this invoice item has been created for, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Expandable<Subscription>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_item: Option<String>,

    /// The tax rates which apply to the invoice item.
    ///
    /// When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<TaxRate>>,

    /// Unit Amount (in the `currency` specified) of the invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
}

impl InvoiceItem {
    /// Returns a list of your invoice items.
    ///
    /// Invoice items are returned sorted by creation date, with the most recently created invoice items appearing first.
    pub fn list(client: &Client, params: ListInvoiceItems<'_>) -> Response<List<InvoiceItem>> {
        client.get_query("/invoiceitems", &params)
    }

    /// Creates an item to be added to a draft invoice.
    ///
    /// If no invoice is specified, the item will be on the next invoice created for the customer specified.
    pub fn create(client: &Client, params: CreateInvoiceItem<'_>) -> Response<InvoiceItem> {
        client.post_form("/invoiceitems", &params)
    }

    /// Retrieves the invoice item with the given ID.
    pub fn retrieve(client: &Client, id: &InvoiceItemId, expand: &[&str]) -> Response<InvoiceItem> {
        client.get_query(&format!("/invoiceitems/{}", id), &Expand { expand })
    }

    /// Updates the amount or description of an invoice item on an upcoming invoice.
    ///
    /// Updating an invoice item is only possible before the invoice it’s attached to is closed.
    pub fn update(
        client: &Client,
        id: &InvoiceItemId,
        params: UpdateInvoiceItem<'_>,
    ) -> Response<InvoiceItem> {
        client.post_form(&format!("/invoiceitems/{}", id), &params)
    }

    /// Deletes an invoice item, removing it from an invoice.
    ///
    /// Deleting invoice items is only possible when they’re not attached to invoices, or if it’s attached to a draft invoice.
    pub fn delete(client: &Client, id: &InvoiceItemId) -> Response<Deleted<InvoiceItemId>> {
        client.delete(&format!("/invoiceitems/{}", id))
    }
}

impl Object for InvoiceItem {
    type Id = InvoiceItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "invoiceitem"
    }
}

/// The parameters for `InvoiceItem::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateInvoiceItem<'a> {
    /// The integer amount in **%s** of the charge to be applied to the upcoming invoice.
    ///
    /// If you want to apply a credit to the customer's account, pass a negative amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The ID of the customer who will be billed when this invoice item is billed.
    pub customer: CustomerId,

    /// An arbitrary string which you can attach to the invoice item.
    ///
    /// The description is displayed in the invoice for easy tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// Controls whether discounts apply to this invoice item.
    ///
    /// Defaults to false for prorations or negative invoice items, and true for all other invoice items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The ID of an existing invoice to add this invoice item to.
    ///
    /// When left blank, the invoice item will be added to the next upcoming scheduled invoice.
    /// This is useful when adding invoice items in response to an invoice.created webhook.
    /// You can only add invoice items to draft invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<InvoiceId>,

    /// A set of key-value pairs that you can attach to an invoice item object.
    ///
    /// It can be useful for storing additional information about the invoice item in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The period associated with this invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,

    /// Non-negative integer.
    ///
    /// The quantity of units for the invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The ID of a subscription to add this invoice item to.
    ///
    /// When left blank, the invoice item will be be added to the next upcoming scheduled invoice.
    /// When set, scheduled invoices for subscriptions other than the specified subscription will ignore the invoice item.
    /// Use this when you want to express that an invoice item has been accrued within the context of a particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<SubscriptionId>,

    /// The tax rates which apply to the invoice item.
    ///
    /// When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,

    /// The integer unit amount in **%s** of the charge to be applied to the upcoming invoice.
    ///
    /// This unit_amount will be multiplied by the quantity to get the full amount.
    /// If you want to apply a credit to the customer's account, pass a negative unit_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
}

impl<'a> CreateInvoiceItem<'a> {
    pub fn new(currency: Currency, customer: CustomerId) -> Self {
        CreateInvoiceItem {
            amount: Default::default(),
            currency,
            customer,
            description: Default::default(),
            discountable: Default::default(),
            expand: Default::default(),
            invoice: Default::default(),
            metadata: Default::default(),
            period: Default::default(),
            quantity: Default::default(),
            subscription: Default::default(),
            tax_rates: Default::default(),
            unit_amount: Default::default(),
        }
    }
}

/// The parameters for `InvoiceItem::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListInvoiceItems<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// The identifier of the customer whose invoice items to return.
    ///
    /// If none is provided, all invoice items will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<InvoiceItemId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Only return invoice items belonging to this invoice.
    ///
    /// If none is provided, all invoice items will be returned.
    /// If specifying an invoice, no customer identifier is needed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<InvoiceId>,

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Set to `true` to only show pending invoice items, which are not yet attached to any invoices.
    ///
    /// Set to `false` to only show invoice items already attached to invoices.
    /// If unspecified, no filter is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<InvoiceItemId>,
}

impl<'a> ListInvoiceItems<'a> {
    pub fn new() -> Self {
        ListInvoiceItems {
            created: Default::default(),
            customer: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            invoice: Default::default(),
            limit: Default::default(),
            pending: Default::default(),
            starting_after: Default::default(),
        }
    }
}

/// The parameters for `InvoiceItem::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateInvoiceItem<'a> {
    /// The integer amount in **%s** of the charge to be applied to the upcoming invoice.
    ///
    /// If you want to apply a credit to the customer's account, pass a negative amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// An arbitrary string which you can attach to the invoice item.
    ///
    /// The description is displayed in the invoice for easy tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// Controls whether discounts apply to this invoice item.
    ///
    /// Defaults to false for prorations or negative invoice items, and true for all other invoice items.
    /// Cannot be set to true for prorations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A set of key-value pairs that you can attach to an invoice item object.
    ///
    /// It can be useful for storing additional information about the invoice item in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The period associated with this invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,

    /// Non-negative integer.
    ///
    /// The quantity of units for the invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The tax rates which apply to the invoice item.
    ///
    /// When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,

    /// The integer unit amount in **%s** of the charge to be applied to the upcoming invoice.
    ///
    /// This unit_amount will be multiplied by the quantity to get the full amount.
    /// If you want to apply a credit to the customer's account, pass a negative unit_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
}

impl<'a> UpdateInvoiceItem<'a> {
    pub fn new() -> Self {
        UpdateInvoiceItem {
            amount: Default::default(),
            description: Default::default(),
            discountable: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
            period: Default::default(),
            quantity: Default::default(),
            tax_rates: Default::default(),
            unit_amount: Default::default(),
        }
    }
}
