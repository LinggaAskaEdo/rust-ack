use bcrypt::{DEFAULT_COST, hash, verify};

fn main() {
    let password = "password123";
    let hashed = hash(password, DEFAULT_COST).unwrap();
    let stored_hash = "$2b$12$02G60MOFy4bqnh0FFitgae9yWoSRAVwSu..T4c4U5CqTqwUGfJk3m"; // Example hash

    println!();
    println!("Password: {}", password);
    println!("Bcrypt Hash: {}", hashed);
    println!("Stored Hash: {}", stored_hash);
    println!();
    println!("Use this hash in your SQL INSERT statement:");
    println!("INSERT INTO users (id, username, email, password_hash) VALUES");
    println!(
        "('550e8400-e29b-41d4-a716-446655440000', 'admin', 'admin@example.com', '{}');",
        hashed
    );
    println!();

    let err = verify(password, &hashed).unwrap();
    println!("Password verification hashed: {}", err);

    let err = verify(password, stored_hash).unwrap();
    println!("Password verification stored: {}", err);
}
