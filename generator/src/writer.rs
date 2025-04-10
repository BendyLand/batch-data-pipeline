use arrow2::array::*;
use std::fs::File;
use std::sync::Arc;
use std::io::BufWriter;
use arrow2::chunk::Chunk;
use arrow2::datatypes::*;
use arrow2::io::parquet::write::*;
use arrow2::io::parquet::write::to_parquet_leaves;
use arrow2::io::parquet::write::CompressionOptions;
use arrow2::array::{Array, Int64Array, StructArray, Utf8Array};
use arrow2::datatypes::{DataType, Field};

use crate::orders::Order;

/// Convert a list of orders into an Arrow Chunk (table-like columnar batch)
pub fn orders_to_chunk(orders: &[Order]) -> Chunk<Arc<dyn Array>> {
    let id_array = Utf8Array::<i32>::from_slice(orders.iter().map(|o| o.id.as_str()).collect::<Vec<_>>());
    let customer_array = get_customer_array(orders);
    let product_array = get_product_array(orders);
    let payment_array = get_payment_array(orders);
    let discount_array = Float64Array::from_iter(orders.iter().map(|o| Some(o.discount)));
    let quantity_array = UInt32Array::from_iter(orders.iter().map(|o| Some(o.quantity)));
    let total_array = Float64Array::from_iter(orders.iter().map(|o| Some(o.total)));
    let date_array = Utf8Array::<i32>::from_slice(orders.iter().map(|o| o.date.to_rfc3339()).collect::<Vec<_>>());
    let status_array = Utf8Array::<i32>::from_slice(orders.iter().map(|o| format!("{:?}", o.status)).collect::<Vec<_>>());

    Chunk::new(vec![
        Arc::new(id_array),
        customer_array,
        product_array,
        payment_array,
        Arc::new(discount_array),
        Arc::new(quantity_array),
        Arc::new(total_array),
        Arc::new(date_array),
        Arc::new(status_array),
    ])
}

pub fn get_payment_array(orders: &[Order]) -> Arc<dyn Array> {
    let transaction_id_array = Utf8Array::<i32>::from_slice(
        orders.iter().map(|o| o.payment.transaction_id.as_str()).collect::<Vec<_>>(),
    );

    let expiration_array = Utf8Array::<i32>::from_slice(
        orders
            .iter()
            .map(|o| o.payment.details.expiration_date().to_string())
            .collect::<Vec<_>>(),
    );

    let struct_array = StructArray::new(
        DataType::Struct(vec![
            Field::new("transaction_id", DataType::Utf8, false),
            Field::new("expiration", DataType::Utf8, false),
        ]),
        vec![
            Box::new(transaction_id_array) as Box<dyn Array>,
            Box::new(expiration_array),
        ],
        None,
    );

    Arc::new(struct_array)
}

pub fn get_product_array(orders: &[Order]) -> Arc<dyn Array> {
    let product_id_array = Int64Array::from_slice(
        orders.iter().map(|o| o.product.id).collect::<Vec<_>>(),
    );

    let product_name_array = Utf8Array::<i32>::from_slice(
        orders.iter().map(|o| o.product.name.as_str()).collect::<Vec<_>>(),
    );

    let product_category_array = Utf8Array::<i32>::from_slice(
        orders.iter().map(|o| format!("{}", o.product.category)).collect::<Vec<_>>(),
    );

    let product_price_array = Float64Array::from_iter(
        orders.iter().map(|o| Some(o.product.price)),
    );

    let struct_array = StructArray::new(
        DataType::Struct(vec![
            Field::new("id", DataType::Int64, false),
            Field::new("name", DataType::Utf8, false),
            Field::new("category", DataType::Utf8, false),
            Field::new("price", DataType::Float64, false),
        ]),
        vec![
            Box::new(product_id_array) as Box<dyn Array>,
            Box::new(product_name_array),
            Box::new(product_category_array),
            Box::new(product_price_array),
        ],
        None,
    );

    Arc::new(struct_array)
}



pub fn get_customer_array(orders: &[Order]) -> Arc<dyn Array> {
    let customer_id_array = Int64Array::from_slice(orders.iter().map(|o| o.customer.id).collect::<Vec<_>>());
    let customer_name_array = Utf8Array::<i32>::from_slice(orders.iter().map(|o| o.customer.name.as_str()).collect::<Vec<_>>());
    let customer_email_array = Utf8Array::<i32>::from_slice(orders.iter().map(|o| o.customer.email.as_str()).collect::<Vec<_>>());
    let customer_address_array = Utf8Array::<i32>::from_slice(orders.iter().map(|o| o.customer.address.as_str()).collect::<Vec<_>>());
    let customer_status_array = Utf8Array::<i32>::from_slice(orders.iter().map(|o| format!("{:?}", o.customer.status)).collect::<Vec<_>>());
    let customer_array = StructArray::new(
        DataType::Struct(vec![
            Field::new("id", DataType::Int64, false),
            Field::new("name", DataType::Utf8, false),
            Field::new("email", DataType::Utf8, false),
            Field::new("address", DataType::Utf8, false),
            Field::new("status", DataType::Utf8, false),
        ]),
        vec![
            Box::new(customer_id_array),
            Box::new(customer_name_array),
            Box::new(customer_email_array),
            Box::new(customer_address_array),
            Box::new(customer_status_array),
        ],
        None,
    );
    Arc::new(customer_array)
}

pub fn get_order_schema() -> Schema {
    Schema::from(vec![
        Field::new("id", DataType::Utf8, false),
        Field::new("customer", DataType::Struct(vec![
            Field::new("id", DataType::Int64, false),
            Field::new("name", DataType::Utf8, false),
            Field::new("email", DataType::Utf8, false),
            Field::new("address", DataType::Utf8, false),
            Field::new("status", DataType::Utf8, false),
        ]), false),
        Field::new("product", DataType::Struct(vec![
            Field::new("id", DataType::Int64, false),
            Field::new("name", DataType::Utf8, false),
            Field::new("category", DataType::Utf8, false),
            Field::new("price", DataType::Float64, false),
        ]), false),
        Field::new("payment", DataType::Struct(vec![
            Field::new("transaction_id", DataType::Utf8, false),
            Field::new("expiration", DataType::Utf8, false),
        ]), false),
        Field::new("discount", DataType::Float64, true),
        Field::new("quantity", DataType::UInt32, false),
        Field::new("total", DataType::Float64, false),
        Field::new("date", DataType::Utf8, false),
        Field::new("status", DataType::Utf8, false),
    ])
}

pub fn write_parquet(orders: &[Order], output_path: &str) -> arrow2::error::Result<()> {
    let schema = get_order_schema();
    let chunk = orders_to_chunk(orders);

    let options = WriteOptions {
        write_statistics: true,
        compression: CompressionOptions::Zstd(Some(ZstdLevel::try_new(3).unwrap())),
        version: Version::V2,
        data_pagesize_limit: None,
    };

    let encodings: Vec<Vec<Encoding>> = schema
        .fields
        .iter()
        .map(|field| {
            let parquet_type = to_parquet_type(field).unwrap();
            let leaf_count = to_parquet_leaves(parquet_type).len();
            vec![Encoding::Plain; leaf_count]
        })
        .collect();


    let mut row_groups = RowGroupIterator::try_new(
        std::iter::once(Ok(chunk)),
        &schema,
        options,
        encodings,
    )?;

    let file = File::create(output_path)?;
    let mut writer = FileWriter::try_new(BufWriter::new(file), schema, options)?;

    for group in &mut row_groups {
        writer.write(group?)?;
    }

    writer.end(None)?;
    Ok(())
}


