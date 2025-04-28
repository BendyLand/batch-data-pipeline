package products

import (
	"encoding/json"
	"generator/utils"
	"math/rand"
)

type ProductCategory int

const (
	Grocery ProductCategory = iota
	HealthAndWellness
	CleaningSupplies
	PetSupplies
	OfficeSupplies
	ToysAndGames
	SeasonalItems
	Clothing
	Electronics
	Furniture
	Kitchenware
	Lighting
	BeddingAndBath
	Tools
	OutdoorEquipment
	Automotive
)

var productCategoryNames = map[ProductCategory]string{
	Grocery:           "Grocery",
	HealthAndWellness: "Health and Wellness",
	CleaningSupplies:  "Cleaning Supplies",
	PetSupplies:       "Pet Supplies",
	OfficeSupplies:    "Office Supplies",
	ToysAndGames:      "Toys and Games",
	SeasonalItems:     "Seasonal Items",
	Clothing:          "Clothing",
	Electronics:       "Electronics",
	Furniture:         "Furniture",
	Kitchenware:       "Kitchenware",
	Lighting:          "Lighting",
	BeddingAndBath:    "Bedding and Bath",
	Tools:             "Tools",
	OutdoorEquipment:  "Outdoor Equipment",
	Automotive:        "Automotive",
}

func (pc ProductCategory) String() string {
	return productCategoryNames[pc]
}

func (s ProductCategory) MarshalJSON() ([]byte, error) {
	return json.Marshal(s.String())
}

type Product struct {
	Id       int64
	Name     string
	Category ProductCategory
	Price    float64
}

var productPrices = map[int64]float64{
	0:  3.99,
	1:  3.49,
	2:  0.89,
	3:  2.99,
	4:  3.99,
	5:  9.99,
	6:  3.00,
	7:  14.99,
	8:  4.99,
	9:  3.99,
	10: 5.49,
	11: 30.00,
	12: 17.49,
	13: 12.99,
	14: 5.00,
	15: 3.49,
	16: 21.99,
	17: 16.79,
	18: 18.99,
	19: 9.89,
	20: 14.99,
	21: 21.19,
	22: 10.00,
	23: 14.00,
	24: 29.99,
	25: 28.00,
	26: 149.99,
	27: 5.99,
	28: 349.99,
	29: 77.99,
	30: 259.00,
	31: 189.99,
	32: 33.49,
	33: 39.99,
	34: 79.99,
	35: 90.00,
	36: 59.99,
	37: 9.99,
	38: 69.99,
	39: 44.99,
	40: 35.00,
	41: 89.99,
	42: 29.99,
	43: 9.99,
	44: 119.99,
	45: 29.99,
	46: 45.00,
	47: 14.29,
	48: 23.49,
	49: 2.59,
}

var productIds = map[int64]string{
	0:  "Whole Wheat Bread", // grocery
	1:  "Whole Milk (1 Gallon)",
	2:  "Canned Black Beans",
	3:  "White Rice (2 lb bag)",
	4:  "Ibuprofen (200mg, 100ct)", // health and wellness
	5:  "Multivitamins (Adult)",
	6:  "Hand Sanitizer (12oz)",
	7:  "Digital Thermometer",
	8:  "All-Purpose Cleaner", // cleaning supplies
	9:  "Dishwashing Liquid (32oz)",
	10: "Disinfecting Wipes (70ct)",
	11: "Dry Dog Food (15lb)", // pet supplies
	12: "Cat Litter (25lb)",
	13: "Pet Shampoo",
	14: "Ballpoint pen (10-pack)", // office supplies
	15: "Spiral Notebook (college ruled)",
	16: "Inkjet Printer Ink Cartridge (color)",
	17: "Building Blocks Set", // toys and games
	18: "Puzzle (1000 pieces)",
	19: "Action Figure",
	20: "LED Christmas Lights (100ct)", // seasonal items
	21: "Halloween Pumpkin Carving Kit",
	22: "Summer Beach Towel",
	23: "Men's Graphic T-Shirt", // clothing
	24: "Women's Yoga Pants",
	25: "Children's Raincoat",
	26: "Wireless Bluetooth Speaker", // electronics
	27: "USB-C Charging Cable",
	28: "Noise-Canceling Headphones",
	29: "4-Tier Bookshelf", // furniture
	30: "Accent Chair -- Wicker",
	31: "Folding Dining Table",
	32: "Non-stick Frying Pan (10\")", // kitchenware
	33: "Stainless Steel Mixing Bowls (3-pack)",
	34: "Chef's Knife (8\")",
	35: "LED Floor Lamp", // lighting
	36: "Smart Light Bulbs (4-pack)",
	37: "Motion Sensor Night Light",
	38: "Queen Comforter Set", // bedding and bath
	39: "Bath Towel Set (6-piece)",
	40: "Memory Foam Pillow",
	41: "Cordless Power Drill (18V)", // tools
	42: "Adjustable Wrench Set",
	43: "Tape Measure (25ft)",
	44: "Charcoal Grill", // outdoor equipment
	45: "Garden Hose (50ft)",
	46: "Outdoor Solar Path Lights (6-pack)",
	47: "Motor Oil (5W-30, 5qt)", // automotive
	48: "Windshield Wipers (front pair)",
	49: "Car Air Freshener -- Pine Scent",
}

func getRandomId() int64 {
	return int64(rand.Intn(50))
}

func getName(id int64) string {
	return productIds[id]
}

func getCategory(id int64) ProductCategory {
	var result ProductCategory
	switch id {
	case 0, 1, 2, 3:
		result = Grocery
	case 4, 5, 6, 7:
		result = HealthAndWellness
	case 8, 9, 10:
		result = CleaningSupplies
	case 11, 12, 13:
		result = PetSupplies
	case 14, 15, 16:
		result = OfficeSupplies
	case 17, 18, 19:
		result = ToysAndGames
	case 20, 21, 22:
		result = SeasonalItems
	case 23, 24, 25:
		result = Clothing
	case 26, 27, 28:
		result = Electronics
	case 29, 30, 31:
		result = Furniture
	case 32, 33, 34:
		result = Kitchenware
	case 35, 36, 37:
		result = Lighting
	case 38, 39, 40:
		result = BeddingAndBath
	case 41, 42, 43:
		result = Tools
	case 44, 45, 46:
		result = OutdoorEquipment
	case 47, 48, 49:
		result = Automotive
	}
	return result
}

func getPrice(id int64) float64 {
	return utils.RoundDecimal(productPrices[id])
}

func GenerateProduct() Product {
	id := getRandomId()
	return Product{
		Id:       id,
		Name:     getName(id),
		Category: getCategory(id),
		Price:    utils.RoundDecimal(getPrice(id)),
	}
}
