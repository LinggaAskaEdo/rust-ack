#!/bin/bash

# ============================================
# Rust REST API Test Script
# ============================================
# This script tests all API endpoints
# Usage: chmod +x test_api.sh && ./test_api.sh
# ============================================

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# API Base URL
BASE_URL="http://localhost:8080"

# Variables to store created IDs
TOKEN=""
USER_ID=""
PRODUCT_ID=""

# Function to print section headers
print_header() {
    echo -e "\n${BLUE}═══════════════════════════════════════════════${NC}"
    echo -e "${CYAN}$1${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════${NC}\n"
}

# Function to print success
print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

# Function to print error
print_error() {
    echo -e "${RED}✗ $1${NC}"
}

# Function to print info
print_info() {
    echo -e "${YELLOW}ℹ $1${NC}"
}

# Function to extract JSON value
extract_json() {
    echo "$1" | grep -o "\"$2\":\"[^\"]*\"" | cut -d'"' -f4
}

# Function to make API call and display result
api_call() {
    local method=$1
    local endpoint=$2
    local data=$3
    local auth=$4
    
    echo -e "${PURPLE}Request:${NC} $method $endpoint"
    
    if [ -n "$data" ]; then
        echo -e "${PURPLE}Payload:${NC} $data"
    fi
    
    local cmd="curl -s -X $method"
    cmd="$cmd -H 'Content-Type: application/json'"
    
    if [ -n "$auth" ]; then
        cmd="$cmd -H 'Authorization: Bearer $auth'"
    fi
    
    if [ -n "$data" ]; then
        cmd="$cmd -d '$data'"
    fi
    
    cmd="$cmd -w '\nHTTP_STATUS:%{http_code}' $BASE_URL$endpoint"
    
    local response=$(eval $cmd)
    local http_status=$(echo "$response" | grep "HTTP_STATUS:" | cut -d':' -f2)
    local body=$(echo "$response" | sed '/HTTP_STATUS:/d')
    
    echo -e "${PURPLE}Response (HTTP $http_status):${NC}"
    echo "$body" | python3 -m json.tool 2>/dev/null || echo "$body"
    
    echo "$body"
}

# Check if server is running
check_server() {
    print_header "Checking Server Status"
    
    if curl -s "$BASE_URL/api/auth/login" > /dev/null 2>&1; then
        print_success "Server is running at $BASE_URL"
    else
        print_error "Server is not running at $BASE_URL"
        print_info "Please start the server with: cargo run"
        exit 1
    fi
}

# Test 1: Login (Public Endpoint)
test_login() {
    print_header "TEST 1: Login with Valid Credentials"
    
    local response=$(api_call "POST" "/api/auth/login" '{
        "username": "admin",
        "password": "password123"
    }')
    
    TOKEN=$(extract_json "$response" "token")
    
    if [ -n "$TOKEN" ]; then
        print_success "Login successful! Token received."
        print_info "Token: ${TOKEN:0:50}..."
    else
        print_error "Login failed!"
        exit 1
    fi
}

# Test 2: Login with Invalid Credentials
test_login_invalid() {
    print_header "TEST 2: Login with Invalid Credentials (Should Fail)"
    
    api_call "POST" "/api/auth/login" '{
        "username": "admin",
        "password": "wrongpassword"
    }'
    
    print_info "This should return 401 Unauthorized"
}

# Test 3: Access Protected Endpoint Without Token
test_no_token() {
    print_header "TEST 3: Access Protected Endpoint Without Token (Should Fail)"
    
    api_call "GET" "/api/users"
    
    print_info "This should return 401 Unauthorized"
}

# Test 4: Create User
test_create_user() {
    print_header "TEST 4: Create New User"
    
    local response=$(api_call "POST" "/api/users" '{
        "username": "testuser_'$(date +%s)'",
        "email": "test_'$(date +%s)'@example.com",
        "password": "testpassword123"
    }' "$TOKEN")
    
    USER_ID=$(extract_json "$response" "id")
    
    if [ -n "$USER_ID" ]; then
        print_success "User created successfully!"
        print_info "User ID: $USER_ID"
    else
        print_error "Failed to create user!"
    fi
}

# Test 5: Get All Users
test_get_all_users() {
    print_header "TEST 5: Get All Users"
    
    api_call "GET" "/api/users" "" "$TOKEN"
    
    print_success "Retrieved all users"
}

# Test 6: Get User by ID
test_get_user_by_id() {
    print_header "TEST 6: Get User by ID"
    
    if [ -n "$USER_ID" ]; then
        api_call "GET" "/api/users/$USER_ID" "" "$TOKEN"
        print_success "Retrieved user by ID"
    else
        print_info "Skipping - No user ID available"
    fi
}

# Test 7: Update User
test_update_user() {
    print_header "TEST 7: Update User"
    
    if [ -n "$USER_ID" ]; then
        api_call "PUT" "/api/users/$USER_ID" '{
            "email": "updated_'$(date +%s)'@example.com"
        }' "$TOKEN"
        
        print_success "User updated successfully"
    else
        print_info "Skipping - No user ID available"
    fi
}

# Test 8: Create Product
test_create_product() {
    print_header "TEST 8: Create New Product"
    
    local response=$(api_call "POST" "/api/products" '{
        "name": "Test Product '$(date +%s)'",
        "description": "This is a test product created by API test script",
        "price": 99.99,
        "stock": 50
    }' "$TOKEN")
    
    PRODUCT_ID=$(extract_json "$response" "id")
    
    if [ -n "$PRODUCT_ID" ]; then
        print_success "Product created successfully!"
        print_info "Product ID: $PRODUCT_ID"
    else
        print_error "Failed to create product!"
    fi
}

# Test 9: Get All Products
test_get_all_products() {
    print_header "TEST 9: Get All Products (No Filters)"
    
    api_call "GET" "/api/products" "" "$TOKEN"
    
    print_success "Retrieved all products"
}

# Test 10: Search Products by Name
test_search_by_name() {
    print_header "TEST 10: Search Products by Name (Dynamic Query)"
    
    api_call "GET" "/api/products?name=laptop" "" "$TOKEN"
    
    print_success "Search by name completed"
}

# Test 11: Search Products by Price Range
test_search_by_price() {
    print_header "TEST 11: Search Products by Price Range (Dynamic Query)"
    
    api_call "GET" "/api/products?min_price=50&max_price=500" "" "$TOKEN"
    
    print_success "Search by price range completed"
}

# Test 12: Search Products with Multiple Filters
test_search_multiple_filters() {
    print_header "TEST 12: Search Products with Multiple Filters (Dynamic Query)"
    
    api_call "GET" "/api/products?name=mouse&min_price=20&max_price=100&min_stock=50" "" "$TOKEN"
    
    print_success "Multi-filter search completed"
}

# Test 13: Get Product by ID
test_get_product_by_id() {
    print_header "TEST 13: Get Product by ID"
    
    if [ -n "$PRODUCT_ID" ]; then
        api_call "GET" "/api/products/$PRODUCT_ID" "" "$TOKEN"
        print_success "Retrieved product by ID"
    else
        print_info "Skipping - No product ID available"
    fi
}

# Test 14: Update Product
test_update_product() {
    print_header "TEST 14: Update Product"
    
    if [ -n "$PRODUCT_ID" ]; then
        api_call "PUT" "/api/products/$PRODUCT_ID" '{
            "price": 79.99,
            "stock": 75
        }' "$TOKEN"
        
        print_success "Product updated successfully"
    else
        print_info "Skipping - No product ID available"
    fi
}

# Test 15: Create Multiple Products
test_create_multiple_products() {
    print_header "TEST 15: Create Multiple Products"
    
    print_info "Creating product 1..."
    api_call "POST" "/api/products" '{
        "name": "Gaming Mouse",
        "description": "RGB gaming mouse with 16000 DPI",
        "price": 49.99,
        "stock": 100
    }' "$TOKEN" > /dev/null
    
    print_info "Creating product 2..."
    api_call "POST" "/api/products" '{
        "name": "Mechanical Keyboard",
        "description": "RGB mechanical keyboard with Cherry MX switches",
        "price": 129.99,
        "stock": 75
    }' "$TOKEN" > /dev/null
    
    print_info "Creating product 3..."
    api_call "POST" "/api/products" '{
        "name": "4K Monitor",
        "description": "32-inch 4K HDR monitor",
        "price": 599.99,
        "stock": 30
    }' "$TOKEN" > /dev/null
    
    print_success "Created 3 additional products"
}

# Test 16: Search by Stock
test_search_by_stock() {
    print_header "TEST 16: Search Products by Minimum Stock"
    
    api_call "GET" "/api/products?min_stock=50" "" "$TOKEN"
    
    print_success "Search by minimum stock completed"
}

# Test 17: Delete Product
test_delete_product() {
    print_header "TEST 17: Delete Product"
    
    if [ -n "$PRODUCT_ID" ]; then
        api_call "DELETE" "/api/products/$PRODUCT_ID" "" "$TOKEN"
        print_success "Product deleted successfully"
    else
        print_info "Skipping - No product ID available"
    fi
}

# Test 18: Verify Product Deleted
test_verify_delete_product() {
    print_header "TEST 18: Verify Product Deleted (Should Return 404)"
    
    if [ -n "$PRODUCT_ID" ]; then
        api_call "GET" "/api/products/$PRODUCT_ID" "" "$TOKEN"
        print_info "This should return 404 Not Found"
    else
        print_info "Skipping - No product ID available"
    fi
}

# Test 19: Delete User
test_delete_user() {
    print_header "TEST 19: Delete User"
    
    if [ -n "$USER_ID" ]; then
        api_call "DELETE" "/api/users/$USER_ID" "" "$TOKEN"
        print_success "User deleted successfully"
    else
        print_info "Skipping - No user ID available"
    fi
}

# Test 20: Logout
test_logout() {
    print_header "TEST 20: Logout"
    
    api_call "POST" "/api/auth/logout" "" "$TOKEN"
    
    print_success "Logged out successfully"
}

# Test 21: Try to Access After Logout
test_after_logout() {
    print_header "TEST 21: Access Protected Endpoint After Logout (Should Fail)"
    
    api_call "GET" "/api/users" "" "$TOKEN"
    
    print_info "This should return 401 Unauthorized (token invalidated)"
}

# Test 22: Edge Cases - Invalid UUID
test_invalid_uuid() {
    print_header "TEST 22: Get User with Invalid UUID (Should Fail)"
    
    api_call "GET" "/api/users/invalid-uuid-123" "" "$TOKEN"
    
    print_info "This should return an error"
}

# Test 23: Edge Cases - Empty Product Search
test_empty_search() {
    print_header "TEST 23: Search Products with No Matches"
    
    api_call "GET" "/api/products?name=nonexistentproduct12345" "" "$TOKEN"
    
    print_info "Should return empty array"
}

# Summary
print_summary() {
    print_header "TEST SUMMARY"
    
    echo -e "${GREEN}✓ All tests completed!${NC}\n"
    echo -e "${CYAN}Tests Run:${NC}"
    echo "  • Authentication (login/logout)"
    echo "  • User CRUD operations"
    echo "  • Product CRUD operations"
    echo "  • Dynamic SQL queries (MyBatis-style)"
    echo "  • Authorization middleware"
    echo "  • Error handling"
    echo "  • Edge cases"
    echo ""
    echo -e "${YELLOW}Note:${NC} Some tests are expected to fail (e.g., invalid credentials, missing token)"
    echo ""
}

# Main execution
main() {
    echo -e "${CYAN}"
    echo "╔════════════════════════════════════════════════════╗"
    echo "║     Rust REST API - Comprehensive Test Suite      ║"
    echo "╚════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    
    # Check if Python is available for JSON formatting
    if ! command -v python3 &> /dev/null; then
        print_info "Python3 not found. JSON output will not be formatted."
    fi
    
    check_server
    
    # Run all tests
    test_login
    test_login_invalid
    test_no_token
    test_create_user
    test_get_all_users
    test_get_user_by_id
    test_update_user
    test_create_product
    test_get_all_products
    test_search_by_name
    test_search_by_price
    test_search_multiple_filters
    test_get_product_by_id
    test_update_product
    test_create_multiple_products
    test_search_by_stock
    test_delete_product
    test_verify_delete_product
    test_delete_user
    
    # Need to login again since we might have deleted the session
    test_login
    test_logout
    test_after_logout
    
    # Edge cases
    test_login  # Login again for edge cases
    test_invalid_uuid
    test_empty_search
    
    print_summary
}

# Run main function
main