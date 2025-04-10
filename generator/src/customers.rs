use rand::Rng;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy)]
#[allow(dead_code)]
pub enum CustomerStatus {
    NewCustomer,
    ReturningCustomer,
    RewardsMember,
    Employee,
    Manager,
    Owner,
}

#[derive(Debug, Serialize, Clone)]
pub struct Customer {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub address: String,
    pub status: CustomerStatus,
}

pub fn generate_customer() -> Customer {
    let name = get_name();
    return Customer {
        id: get_customer_id(&name),
        name: name.clone(),
        email: get_email(&name),
        address: get_address(&name),
        status: get_status(&name),
    };
}

// ------------------------ Data + Logic ------------------------

fn get_name() -> String {
    let names = NAMES;
    let mut rng = rand::rng();

    let mut choice: usize = rng.random_range(0..25);
    if rng.random_range(0..5) == 0 {
        // 20% chance to shift index
        choice = choice.saturating_sub(rng.random_range(0..10));
    }
    return names[choice].to_string();
}

fn get_customer_id(name: &str) -> i64 {
    return {
        CUSTOMER_IDS
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, id)| *id).unwrap_or(-1)
    };
}

fn get_email(name: &str) -> String {
    return {
        CUSTOMER_EMAILS
            .iter()
            .find(|(n, _)| *n == name)
            .unwrap()
            .1
            .to_string()
    };
}

fn get_address(name: &str) -> String {
    return {
        CUSTOMER_ADDRESSES
            .iter()
            .find(|(n, _)| *n == name)
            .unwrap()
            .1
            .to_string()
    };
}

fn get_status(name: &str) -> CustomerStatus {
    return match name {
        "Maya Ellison" | "Noah Blackwood" | "Chloe Hartman" |
        "Lucas Pennington" | "Sofia Langford" | "Oliver Drayton" |
        "Harper Linwood" => {
            CustomerStatus::ReturningCustomer
        },
        "Sebastian Knox" | "Amelia Fairbanks" | "Julian Royce" |
        "Nora Halston" | "Elijah Trent" | "Zoe Merrick" => {
            CustomerStatus::RewardsMember
        },
        "Caleb Winslow" | "Lily Hargrove" | "Milo Carrington" |
        "Aria Templeton" | "Declan Shore" => {
            CustomerStatus::Employee
        },
        "Vivian Leclair" | "Grayson Holt" => {
            CustomerStatus::Manager
        },
        "Clara Redmond" => {
            CustomerStatus::Owner
        },
        _ =>CustomerStatus::NewCustomer,
    };
}

// ------------------------ Static Data ------------------------

const NAMES: [&str; 25] = [
    "Ava Whitaker", "Liam Caldwell", "Isabella Greene", "Ethan Morrell",
    "Maya Ellison", "Noah Blackwood", "Chloe Hartman", "Lucas Pennington",
    "Sofia Langford", "Oliver Drayton", "Harper Linwood", "Sebastian Knox",
    "Amelia Fairbanks", "Julian Royce", "Nora Halston", "Elijah Trent",
    "Zoe Merrick", "Caleb Winslow", "Lily Hargrove", "Milo Carrington",
    "Aria Templeton", "Declan Shore", "Vivian Leclair", "Grayson Holt",
    "Clara Redmond"
];

const CUSTOMER_IDS: [(&str, i64); 25] = [
    ("Ava Whitaker", 24), ("Liam Caldwell", 23), ("Isabella Greene", 22), ("Ethan Morrell", 21),
    ("Maya Ellison", 20), ("Noah Blackwood", 19), ("Chloe Hartman", 18), ("Lucas Pennington", 17),
    ("Sofia Langford", 16), ("Oliver Drayton", 15), ("Harper Linwood", 14), ("Sebastian Knox", 13),
    ("Amelia Fairbanks", 12), ("Julian Royce", 11), ("Nora Halston", 10), ("Elijah Trent", 9),
    ("Zoe Merrick", 8), ("Caleb Winslow", 7), ("Lily Hargrove", 6), ("Milo Carrington", 5),
    ("Aria Templeton", 4), ("Declan Shore", 3), ("Vivian Leclair", 2), ("Grayson Holt", 1),
    ("Clara Redmond", 0),
];

const CUSTOMER_EMAILS: [(&str, &str); 25] = [
    ("Ava Whitaker", "whitaker.a@email.com"),
    ("Liam Caldwell", "caldwelll@email.com"),
    ("Isabella Greene", "isabella_greene@email.com"),
    ("Ethan Morrell", "morrelle@email.com"),
    ("Maya Ellison", "ellison_maya@email.com"),
    ("Noah Blackwood", "blackwood3noah@email.com"),
    ("Chloe Hartman", "chloehartman54321@emailprovider.com"),
    ("Lucas Pennington", "lucky_penny10@upmail.com"),
    ("Sofia Langford", "sofie_lang123@email.com"),
    ("Oliver Drayton", "drayton.o@email.com"),
    ("Harper Linwood", "harper.lindwood@downmail.com"),
    ("Sebastian Knox", "sebknox@email.com"),
    ("Amelia Fairbanks", "the_fairest_of_banks@email.com"),
    ("Julian Royce", "royce_j@email.com"),
    ("Nora Halston", "halston.n@emailprovider.com"),
    ("Elijah Trent", "eli_trent123@upmail.com"),
    ("Zoe Merrick", "zoe.merrick@email.com"),
    ("Caleb Winslow", "employee1@store.com"),
    ("Lily Hargrove", "employee2@store.com"),
    ("Milo Carrington", "employee3@store.com"),
    ("Aria Templeton", "employee4@store.com"),
    ("Declan Shore", "employee5@store.com"),
    ("Vivian Leclair", "manager1@store.com"),
    ("Grayson Holt", "manager2@store.com"),
    ("Clara Redmond", "owner@store.com"),
];

const CUSTOMER_ADDRESSES: [(&str, &str); 25] = [
    ("Ava Whitaker", "1281 Marigold Ln, Boulder, CO 80304"),
    ("Liam Caldwell", "760 Pine Hollow Rd, Albany, NY 12205"),
    ("Isabella Greene", "3947 Sycamore Dr, San Diego, CA 92103"),
    ("Ethan Morrell", "2420 Ridgeway Ave, Memphis, TN 38104"),
    ("Maya Ellison", "87 Hilltop Cir, Eugene, OR 97405"),
    ("Noah Blackwood", "511 Wren St, Madison, WI 53703"),
    ("Chloe Hartman", "19 Cypress View Ct, Tampa, FL 33629"),
    ("Lucas Pennington", "933 Windmere Way, Austin, TX 78731"),
    ("Sofia Langford", "1604 Meadowlark Ln, Salt Lake City, UT 84106"),
    ("Oliver Drayton", "43 Hollow Creek Rd, Durham, NC 27707"),
    ("Harper Linwood", "670 Bramblewood Dr, Indianapolis, IN 46220"),
    ("Sebastian Knox", "3582 Auburn Ridge Ct, Scottsdale, AZ 85251"),
    ("Amelia Fairbanks", "1047 Birch Haven Dr, Minneapolis, MN 55406"),
    ("Julian Royce", "213 Ivy Brook Ln, Charleston, SC 29414"),
    ("Nora Halston", "585 Oak Crest Blvd, Des Moines, IA 50310"),
    ("Elijah Trent", "402 Clearwater Ct, Kansas City, MO 64111"),
    ("Zoe Merrick", "144 Harborstone Dr, Anchorage, AK 99501"),
    ("Caleb Winslow", "318 Willow Bend Dr, Raleigh, NC 27609"),
    ("Lily Hargrove", "4902 Maple Grove Ln, Cary, NC 27513"),
    ("Milo Carrington", "1023 Briarcliff Cir, Durham, NC 27705"),
    ("Aria Templeton", "710 Pine Ridge Dr, Chapel Hill, NC 27514"),
    ("Declan Shore", "855 Brookview Rd, Raleigh, NC 27606"),
    ("Vivian Leclair", "6307 Oakdale Way, Apex, NC 27502"),
    ("Grayson Holt", "214 Forest Glen Ct, Garner, NC 27529"),
    ("Clara Redmond", "1201 Amberwood Dr, Holly Springs, NC 27540"),
];

