-- Drop existing tables if they exist
DROP TABLE IF EXISTS products CASCADE;
DROP TABLE IF EXISTS users CASCADE;

-- Create users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(100) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPZ NOT NULL DEFAULT NOW()
);

-- Create products table
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price DECIMAL(10, 2) NOT NULL CHECK (price >= 0),
    stock INTEGER NOT NULL DEFAULT 0 CHECK (stock >= 0),
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_products_name ON products(name);
CREATE INDEX idx_products_price ON products(price);
CREATE INDEX idx_products_created_by ON products(created_by);

-- Insert sample users (password is "password123" for all)
INSERT INTO users (id, username, email, password_hash) VALUES
('550e8400-e29b-41d4-a716-446655440000', 'admin', 'admin@example.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYPkJNXzTie'),
('550e8400-e29b-41d4-a716-446655440001', 'user1', 'user1@example.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYPkJNXzTie'),
('550e8400-e29b-41d4-a716-446655440002', 'user2', 'user2@example.com', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYPkJNXzTie');

-- Insert sample products
INSERT INTO products (name, description, price, stock, created_by) VALUES
('Laptop', 'High-performance laptop with 16GB RAM and 512GB SSD', 999.99, 50, '550e8400-e29b-41d4-a716-446655440000'),
('Mouse', 'Wireless ergonomic mouse', 29.99, 200, '550e8400-e29b-41d4-a716-446655440000'),
('Keyboard', 'Mechanical RGB keyboard', 79.99, 150, '550e8400-e29b-41d4-a716-446655440000'),
('Monitor', '27-inch 4K monitor', 399.99, 75, '550e8400-e29b-41d4-a716-446655440001'),
('Webcam', 'HD 1080p webcam', 59.99, 120, '550e8400-e29b-41d4-a716-446655440001'),
('Headphones', 'Noise-cancelling headphones', 149.99, 90, '550e8400-e29b-41d4-a716-446655440002');
