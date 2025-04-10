use crate::utils::generate_uuid;
use chrono::NaiveDate;
use rand::Rng;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct CardDetails {
    pub number: String,
    pub expiration: NaiveDate,
}

#[derive(Debug, Serialize, Clone)]
pub struct Wallet {
    pub expiration: NaiveDate,
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PaymentDetails {
    Card(CardDetails),
    Wallet(Wallet),
}

impl PaymentDetails {
    pub fn expiration_date(&self) -> NaiveDate {
        return match self {
            PaymentDetails::Card(card) => card.expiration,
            PaymentDetails::Wallet(wallet) => wallet.expiration,
        };
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Payment {
    pub details: PaymentDetails,
    pub transaction_id: String,
}

fn parse_exp_date(exp_str: &str) -> NaiveDate {
    let parts: Vec<u32> = {
        exp_str
            .split('/')
            .map(|s| s.parse().unwrap_or(1))
            .collect()
    };
    let (month, year_suffix) = (parts[0], parts[1]);
    let year = 2000 + year_suffix as i32;

    let first_of_next_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
    } 
    else {
        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
    };
    return first_of_next_month.pred_opt().unwrap();
}

fn get_card_list() -> Vec<CardDetails> {
    return vec![
        ("************5171", "05/25"), ("************7690", "05/27"),
        ("************2763", "06/24"), ("************3315", "04/26"),
        ("************4072", "01/28"), ("************6869", "11/25"),
        ("************2781", "08/26"), ("************0904", "09/25"),
        ("************4469", "09/27"), ("************5164", "08/26"),
        ("************3341", "04/25"), ("************1349", "07/26"),
        ("************4167", "03/25"), ("************7168", "09/25"),
        ("************8239", "10/26"), ("************0932", "11/26"),
        ("************8186", "08/27"), ("************7668", "11/25"),
        ("************9486", "09/27"), ("************1093", "02/27"),
        ("************0238", "01/26"), ("************9611", "02/28"),
        ("************7295", "09/26"), ("************8346", "08/27"),
        ("************2464", "01/26"),
    ].into_iter().map(|(n, d)| CardDetails {
        number: n.to_string(),
        expiration: parse_exp_date(d),
    })
    .collect();
}

fn get_wallet_list() -> Vec<Wallet> {
    return {
        get_card_list()
            .iter()
            .map(|card| Wallet { expiration: card.expiration })
            .collect()
    };
}

pub fn choose_payment_method(name: &str) -> PaymentDetails {
    let cards = get_card_list();
    let wallets = get_wallet_list();
    let name_to_index = [
        "Ava Whitaker", "Liam Caldwell", "Isabella Greene", "Ethan Morrell",
        "Maya Ellison", "Noah Blackwood", "Chloe Hartman", "Lucas Pennington",
        "Sofia Langford", "Oliver Drayton", "Harper Linwood", "Sebastian Knox",
        "Amelia Fairbanks", "Julian Royce", "Nora Halston", "Elijah Trent",
        "Zoe Merrick", "Caleb Winslow", "Lily Hargrove", "Milo Carrington",
        "Aria Templeton", "Declan Shore", "Vivian Leclair", "Grayson Holt",
        "Clara Redmond",
    ];
    if let Some(index) = name_to_index.iter().position(|n| *n == name) {
        let mut rng = rand::rng();
        if rng.random_bool(0.5) { PaymentDetails::Card(cards[index].clone()) } 
        else { PaymentDetails::Wallet(wallets[index].clone()) }
    } 
    else { panic!("Unknown customer name: {name}"); }
}

pub fn new_payment(name: &str) -> Payment {
    return Payment {
        details: choose_payment_method(name),
        transaction_id: generate_uuid(),
    };
}

