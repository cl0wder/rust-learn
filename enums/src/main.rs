struct Product {
    name: String,
    category: ProductCategory,
    price: f32,
    in_stock: bool,
}

enum ProductCategory {
    Books,
    Clothing,
    Electronics,
}

// Database operations implementation
impl Product {
    // Create a new product in the database
    fn create(db: &mut Vec<Product>, product: Product) -> Result<(), &'static str> {
        // Check if product with same name already exists
        if db.iter().any(|p| p.name == product.name) {
            return Err("Product with this name already exists");
        }
        db.push(product);
        Ok(())
    }

    // Read a product by name
    fn find_by_name(db: &[Product], name: &str) -> Option<&Product> {
        db.iter().find(|p| p.name == name)
    }

    // Update a product's price
    fn update_price(db: &mut Vec<Product>, name: &str, new_price: f32) -> Result<(), &'static str> {
        if let Some(product) = db.iter_mut().find(|p| p.name == name) {
            product.price = new_price;
            Ok(())
        } else {
            Err("Product not found")
        }
    }

    // Delete a product by name
    fn delete(db: &mut Vec<Product>, name: &str) -> Result<(), &'static str> {
        if let Some(pos) = db.iter().position(|p| p.name == name) {
            db.remove(pos);
            Ok(())
        } else {
            Err("Product not found")
        }
    }

    // List all products in a category
    fn list_by_category(db: &[Product], category: ProductCategory) -> Vec<&Product> {
        db.iter().filter(|p| matches!(p.category, _ if p.category.to_string() == category.to_string())).collect()
    }

    // Get total value of inventory
    fn get_inventory_value(db: &[Product]) -> f32 {
        db.iter().filter(|p| p.in_stock).map(|p| p.price).sum()
    }
}

// Implementation for Product struct
impl Product {
    // Constructor method (associated function)
    fn new(name: String, category: ProductCategory, price: f32) -> Self {
        Self {
            name,
            category,
            price,
            in_stock: true, // default value
        }
    }

    // Method to check if product is affordable
    fn is_affordable(&self, budget: f32) -> bool {
        self.price <= budget
    }

    // Method to apply discount
    fn apply_discount(&mut self, percentage: f32) {
        self.price = self.price * (1.0 - percentage / 100.0);
    }

    // Method to get product details
    fn get_details(&self) -> String {
        format!("{} ({}): ${:.2}", self.name, self.category.to_string(), self.price)
    }
}

// Implementation for ProductCategory enum
impl ProductCategory {
    // Method to get category description
    fn description(&self) -> &'static str {
        match self {
            ProductCategory::Books => "Books and reading materials",
            ProductCategory::Clothing => "Apparel and accessories",
            ProductCategory::Electronics => "Electronic devices and gadgets",
        }
    }

    // Method to convert category to string
    fn to_string(&self) -> &'static str {
        match self {
            ProductCategory::Books => "Books",
            ProductCategory::Clothing => "Clothing",
            ProductCategory::Electronics => "Electronics",
        }
    }
}

fn main() {
    // Create an in-memory database
    let mut db: Vec<Product> = Vec::new();

    // Create some products
    let tv = Product {
        name: String::from("tv"),
        category: ProductCategory::Electronics,
        price: 133.7,
        in_stock: true,
    };

    let book = Product {
        name: String::from("Rust Programming"),
        category: ProductCategory::Books,
        price: 49.99,
        in_stock: true,
    };

    // Add products to database
    Product::create(&mut db, tv).unwrap();
    Product::create(&mut db, book).unwrap();

    // Find a product
    if let Some(product) = Product::find_by_name(&db, "tv") {
        println!("Found product: {}", product.get_details());
    }

    // Update a product's price
    Product::update_price(&mut db, "tv", 150.0).unwrap();

    // List all electronics
    let electronics = Product::list_by_category(&db, ProductCategory::Electronics);
    println!("\nElectronics in stock:");
    for product in electronics {
        println!("{}", product.get_details());
    }

    // Get total inventory value
    println!("\nTotal inventory value: ${:.2}", Product::get_inventory_value(&db));

    // Delete a product
    Product::delete(&mut db, "tv").unwrap();
    println!("\nAfter deleting tv, total inventory value: ${:.2}", Product::get_inventory_value(&db));
}


