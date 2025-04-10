use rand::Rng;
use serde::Serialize;

use crate::utils::round_decimal;

#[derive(Debug, Serialize, Clone, Copy)]
pub enum ProductCategory {
    Grocery,
    HealthAndWellness,
    CleaningSupplies,
    PetSupplies,
    OfficeSupplies,
    ToysAndGames,
    SeasonalItems,
    Clothing,
    Electronics,
    Furniture,
    Kitchenware,
    Lighting,
    BeddingAndBath,
    Tools,
    OutdoorEquipment,
    Automotive,
}

impl std::fmt::Display for ProductCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ProductCategory::Grocery => "Grocery",
            ProductCategory::HealthAndWellness => "Health and Wellness",
            ProductCategory::CleaningSupplies => "Cleaning Supplies",
            ProductCategory::PetSupplies => "Pet Supplies",
            ProductCategory::OfficeSupplies => "Office Supplies",
            ProductCategory::ToysAndGames => "Toys and Games",
            ProductCategory::SeasonalItems => "Seasonal Items",
            ProductCategory::Clothing => "Clothing",
            ProductCategory::Electronics => "Electronics",
            ProductCategory::Furniture => "Furniture",
            ProductCategory::Kitchenware => "Kitchenware",
            ProductCategory::Lighting => "Lighting",
            ProductCategory::BeddingAndBath => "Bedding and Bath",
            ProductCategory::Tools => "Tools",
            ProductCategory::OutdoorEquipment => "Outdoor Equipment",
            ProductCategory::Automotive => "Automotive",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Serialize)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub category: ProductCategory,
    pub price: f64,
}

pub fn generate_product() -> Product {
    let id = rand::rng().random_range(0..50);
    Product {
        id,
        name: get_name(id),
        category: get_category(id),
        price: round_decimal(get_price(id)),
    }
}

fn get_name(id: i64) -> String {
    PRODUCT_IDS[id as usize].to_string()
}

fn get_category(id: i64) -> ProductCategory {
    match id {
        0..=3 => ProductCategory::Grocery,
        4..=7 => ProductCategory::HealthAndWellness,
        8..=10 => ProductCategory::CleaningSupplies,
        11..=13 => ProductCategory::PetSupplies,
        14..=16 => ProductCategory::OfficeSupplies,
        17..=19 => ProductCategory::ToysAndGames,
        20..=22 => ProductCategory::SeasonalItems,
        23..=25 => ProductCategory::Clothing,
        26..=28 => ProductCategory::Electronics,
        29..=31 => ProductCategory::Furniture,
        32..=34 => ProductCategory::Kitchenware,
        35..=37 => ProductCategory::Lighting,
        38..=40 => ProductCategory::BeddingAndBath,
        41..=43 => ProductCategory::Tools,
        44..=46 => ProductCategory::OutdoorEquipment,
        47..=49 => ProductCategory::Automotive,
        _ => panic!("Invalid product ID"),
    }
}

fn get_price(id: i64) -> f64 {
    PRODUCT_PRICES[id as usize]
}

const PRODUCT_IDS: [&str; 50] = [
    "Whole Wheat Bread", "Whole Milk (1 Gallon)", "Canned Black Beans", "White Rice (2 lb bag)",
    "Ibuprofen (200mg, 100ct)", "Multivitamins (Adult)", "Hand Sanitizer (12oz)", "Digital Thermometer",
    "All-Purpose Cleaner", "Dishwashing Liquid (32oz)", "Disinfecting Wipes (70ct)",
    "Dry Dog Food (15lb)", "Cat Litter (25lb)", "Pet Shampoo",
    "Ballpoint pen (10-pack)", "Spiral Notebook (college ruled)", "Inkjet Printer Ink Cartridge (color)",
    "Building Blocks Set", "Puzzle (1000 pieces)", "Action Figure",
    "LED Christmas Lights (100ct)", "Halloween Pumpkin Carving Kit", "Summer Beach Towel",
    "Men's Graphic T-Shirt", "Women's Yoga Pants", "Children's Raincoat",
    "Wireless Bluetooth Speaker", "USB-C Charging Cable", "Noise-Canceling Headphones",
    "4-Tier Bookshelf", "Accent Chair -- Wicker", "Folding Dining Table",
    "Non-stick Frying Pan (10\")", "Stainless Steel Mixing Bowls (3-pack)", "Chef's Knife (8\")",
    "LED Floor Lamp", "Smart Light Bulbs (4-pack)", "Motion Sensor Night Light",
    "Queen Comforter Set", "Bath Towel Set (6-piece)", "Memory Foam Pillow",
    "Cordless Power Drill (18V)", "Adjustable Wrench Set", "Tape Measure (25ft)",
    "Charcoal Grill", "Garden Hose (50ft)", "Outdoor Solar Path Lights (6-pack)",
    "Motor Oil (5W-30, 5qt)", "Windshield Wipers (front pair)", "Car Air Freshener -- Pine Scent",
];

const PRODUCT_PRICES: [f64; 50] = [
    3.99, 3.49, 0.89, 2.99, 3.99, 9.99, 3.00, 14.99, 4.99, 3.99,
    5.49, 30.00, 17.49, 12.99, 5.00, 3.49, 21.99, 16.79, 18.99, 9.89,
    14.99, 21.19, 10.00, 14.00, 29.99, 28.00, 149.99, 5.99, 349.99, 77.99,
    259.00, 189.99, 33.49, 39.99, 79.99, 90.00, 59.99, 9.99, 69.99, 44.99,
    35.00, 89.99, 29.99, 9.99, 119.99, 29.99, 45.00, 14.29, 23.49, 2.59,
];
