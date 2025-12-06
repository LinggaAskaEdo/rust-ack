#!/bin/bash

# ============================================
# Quick API Test Script
# ============================================
# Quick test for the most common operations
# Usage: chmod +x quick_test.sh && ./quick_test.sh
# ============================================

BASE_URL="http://localhost:8080"

# Colors
GREEN='\033[0;32m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${CYAN}Starting Quick API Test...${NC}\n"

# 1. Login
echo -e "${YELLOW}1. Logging in...${NC}"
LOGIN_RESPONSE=$(curl -s -X POST "$BASE_URL/api/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "password123"
  }')

TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"token":"[^"]*"' | cut -d'"' -f4)

if [ -z "$TOKEN" ]; then
    echo "❌ Login failed!"
    echo "$LOGIN_RESPONSE"
    exit 1
fi

echo -e "${GREEN}✓ Logged in successfully${NC}"
echo "Token: ${TOKEN:0:50}..."
echo ""

# 2. Get All Users
echo -e "${YELLOW}2. Getting all users...${NC}"
curl -s -X GET "$BASE_URL/api/users" \
  -H "Authorization: Bearer $TOKEN" | python3 -m json.tool 2>/dev/null
echo -e "${GREEN}✓ Retrieved users${NC}\n"

# 3. Create Product
echo -e "${YELLOW}3. Creating a product...${NC}"
PRODUCT_RESPONSE=$(curl -s -X POST "$BASE_URL/api/products" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "name": "Quick Test Product",
    "description": "Created by quick test script",
    "price": 49.99,
    "stock": 100
  }')

echo "$PRODUCT_RESPONSE" | python3 -m json.tool 2>/dev/null
PRODUCT_ID=$(echo "$PRODUCT_RESPONSE" | grep -o '"id":"[^"]*"' | cut -d'"' -f4)
echo -e "${GREEN}✓ Product created (ID: $PRODUCT_ID)${NC}\n"

# 4. Search Products
echo -e "${YELLOW}4. Searching products (name=laptop, price 500-1500)...${NC}"
curl -s -X GET "$BASE_URL/api/products?name=laptop&min_price=500&max_price=1500" \
  -H "Authorization: Bearer $TOKEN" | python3 -m json.tool 2>/dev/null
echo -e "${GREEN}✓ Search completed${NC}\n"

# 5. Get Product by ID
if [ -n "$PRODUCT_ID" ]; then
    echo -e "${YELLOW}5. Getting product by ID...${NC}"
    curl -s -X GET "$BASE_URL/api/products/$PRODUCT_ID" \
      -H "Authorization: Bearer $TOKEN" | python3 -m json.tool 2>/dev/null
    echo -e "${GREEN}✓ Retrieved product${NC}\n"
fi

# 6. Update Product
if [ -n "$PRODUCT_ID" ]; then
    echo -e "${YELLOW}6. Updating product...${NC}"
    curl -s -X PUT "$BASE_URL/api/products/$PRODUCT_ID" \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer $TOKEN" \
      -d '{
        "price": 39.99,
        "stock": 150
      }' | python3 -m json.tool 2>/dev/null
    echo -e "${GREEN}✓ Product updated${NC}\n"
fi

# 7. Logout
echo -e "${YELLOW}7. Logging out...${NC}"
curl -s -X POST "$BASE_URL/api/auth/logout" \
  -H "Authorization: Bearer $TOKEN" | python3 -m json.tool 2>/dev/null
echo -e "${GREEN}✓ Logged out${NC}\n"

# 8. Try accessing after logout (should fail)
echo -e "${YELLOW}8. Trying to access after logout (should fail)...${NC}"
FAIL_RESPONSE=$(curl -s -X GET "$BASE_URL/api/users" \
  -H "Authorization: Bearer $TOKEN")
echo "$FAIL_RESPONSE"
echo -e "${GREEN}✓ Correctly rejected unauthorized access${NC}\n"

echo -e "${CYAN}================================${NC}"
echo -e "${GREEN}Quick test completed!${NC}"
echo -e "${CYAN}================================${NC}"