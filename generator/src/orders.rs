use chrono::{DateTime, Utc};
use serde::Serialize;
use std::fmt;

use crate::customers::{self, Customer, CustomerStatus};
use crate::payments::{self, Payment};
use crate::products::{self, Product};
use crate::utils::{generate_datetime, generate_uuid, round_decimal};
use rand::Rng;

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Completed,
    Refunded,
    Cancelled,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OrderStatus::Pending => "Pending",
            OrderStatus::Completed => "Completed",
            OrderStatus::Refunded => "Refunded",
            OrderStatus::Cancelled => "Cancelled",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Serialize)]
pub struct Order {
    pub id: String,
    pub customer: Customer,
    pub product: Product,
    pub date: DateTime<Utc>,
    pub payment: Payment,
    pub status: OrderStatus,
    pub discount: f64,
    pub quantity: u32,
    pub total: f64,
}

// --------------------------------------------

fn get_status(id: &str, payment: &Payment, date: DateTime<Utc>) -> OrderStatus {
    if id.len() != 36 {
        return OrderStatus::Cancelled;
    }

    let now = Utc::now();
    if now.date_naive() > payment.details.expiration_date() {
        return OrderStatus::Cancelled;
    }

    let month_ago = now - chrono::Duration::days(30);
    let week_ago = now - chrono::Duration::days(7);

    if date < month_ago {
        OrderStatus::Completed
    } 
    else if date > month_ago && date < week_ago {
        OrderStatus::Refunded
    } 
    else if date > week_ago && date <= now {
        OrderStatus::Completed
    } 
    else if date > now {
        OrderStatus::Pending
    } 
    else {
        OrderStatus::Completed
    }
}

fn get_quantity() -> u32 {
    let mut rng = rand::rng();
    let temp = rng.random_range(0..3) + rng.random_range(0..2)
        - rng.random_range(0..4) + rng.random_range(0..1);
    if temp > 0 {
        temp as u32
    } 
    else {
        rng.random_range(1..=1)
    }
}

fn get_discount(customer: &Customer) -> f64 {
    let mut rng = rand::rng();
    match customer.status {
        CustomerStatus::NewCustomer => 0.05,
        CustomerStatus::ReturningCustomer => {
            if rng.random_range(0..6) % 2 != 0 {
                0.03
            } 
            else {
                0.0
            }
        }
        CustomerStatus::RewardsMember => 0.10,
        CustomerStatus::Employee => 0.20,
        CustomerStatus::Manager => 0.50,
        CustomerStatus::Owner => 1.00,
    }
}

fn compute_total(order: &mut Order) {
    let raw = order.product.price * order.quantity as f64;
    let discounted = raw * (1.0 - order.discount);
    order.total = round_decimal(discounted);
}

// --------------------------------------------

pub fn generate_order() -> Order {
    let mut id = generate_uuid();
    let date = generate_datetime();
    let product = products::generate_product();
    let customer = customers::generate_customer();
    let payment = payments::new_payment(&customer.name);

    if rand::rng().random_range(0..1000) % 13 == 0 {
        id.push('0'); // corrupt it slightly
    }

    let mut order = Order {
        id: id.clone(),
        date,
        customer,
        product,
        status: get_status(&id, &payment, date),
        payment,
        discount: 0.0, // filled in below
        quantity: get_quantity(),
        total: 0.0,
    };

    order.discount = get_discount(&order.customer);
    compute_total(&mut order);

    order
}

// Optional: for debugging or CLI demo

#[allow(dead_code)]
pub fn show(order: &Order) {
    use std::thread;
    use std::time::Duration;

    println!("start~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~start");
    println!("Order Details:");
    println!(
        "Order ID: {}\nDate: {}\nStatus: {}\nDiscount: {:.2}\nQuantity: {}\nTotal: {:.2}\n",
        order.id, order.date, order.status, order.discount, order.quantity, order.total
    );
    println!("Customer Details:");
    println!(
        "Customer Id: {}\nName: {}\nEmail: {}\nAddress: {}\nStatus: {:?}\n",
        order.customer.id,
        order.customer.name,
        order.customer.email,
        order.customer.address,
        order.customer.status,
    );
    println!("Payment Details:");
    println!(
        "Transaction ID: {}\nExpiration Date: {}\n",
        order.payment.transaction_id,
        order.payment.details.expiration_date()
    );
    println!("Product Details:");
    println!(
        "Product Id: {}\nName: {}\nCategory: {}\nPrice: {:.2}\n",
        order.product.id,
        order.product.name,
        order.product.category,
        order.product.price
    );
    println!("end~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~end");
    thread::sleep(Duration::from_millis(10));
}
