#!/bin/bash

# ============================================
# Load Test Script
# ============================================
# Performs load testing on the API
# Usage: chmod +x load_test.sh && ./load_test.sh [num_requests]
# Example: ./load_test.sh 100
# ============================================

BASE_URL="http://localhost:8080"
NUM_REQUESTS=${1:-50}  # Default to 50 requests if not specified

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}"
echo "╔════════════════════════════════════════════════════╗"
echo "║           API Load Testing Tool                    ║"
echo "╚════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Check if server is running
if ! curl -s "$BASE_URL/api/auth/login" > /dev/null 2>&1; then
    echo -e "${RED}❌ Server is not running at $BASE_URL${NC}"
    echo -e "${YELLOW}Please start the server with: cargo run${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Server is running${NC}\n"

# Login once to get token
echo -e "${YELLOW}Getting authentication token...${NC}"
LOGIN_RESPONSE=$(curl -s -X POST "$BASE_URL/api/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "password123"
  }')

TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"token":"[^"]*"' | cut -d'"' -f4)

if [ -z "$TOKEN" ]; then
    echo -e "${RED}❌ Login failed!${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Authenticated successfully${NC}\n"

# Statistics
SUCCESS_COUNT=0
FAIL_COUNT=0
TOTAL_TIME=0

echo -e "${BLUE}════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}Starting load test with $NUM_REQUESTS requests...${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════${NC}\n"

# Progress bar function
show_progress() {
    local current=$1
    local total=$2
    local width=50
    local percentage=$((current * 100 / total))
    local filled=$((width * current / total))
    local empty=$((width - filled))
    
    printf "\r${YELLOW}Progress: [${NC}"
    printf "%${filled}s" | tr ' ' '█'
    printf "%${empty}s" | tr ' ' '░'
    printf "${YELLOW}] ${percentage}%% (${current}/${total})${NC}"
}

# Test 1: Login Performance
echo -e "${CYAN}Test 1: Login Endpoint Performance${NC}"
LOGIN_START=$(date +%s.%N)
for i in $(seq 1 $NUM_REQUESTS); do
    show_progress $i $NUM_REQUESTS
    
    START=$(date +%s.%N)
    HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/api/auth/login" \
      -H "Content-Type: application/json" \
      -d '{
        "username": "admin",
        "password": "password123"
      }')
    END=$(date +%s.%N)
    
    REQUEST_TIME=$(echo "$END - $START" | bc)
    TOTAL_TIME=$(echo "$TOTAL_TIME + $REQUEST_TIME" | bc)
    
    if [ "$HTTP_CODE" -eq 200 ]; then
        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
    else
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi
done
LOGIN_END=$(date +%s.%N)
LOGIN_DURATION=$(echo "$LOGIN_END - $LOGIN_START" | bc)

echo ""
echo -e "${GREEN}✓ Login test completed${NC}"
echo -e "  Total time: ${YELLOW}${LOGIN_DURATION}s${NC}"
echo -e "  Average response time: ${YELLOW}$(echo "scale=4; $TOTAL_TIME / $NUM_REQUESTS" | bc)s${NC}"
echo -e "  Requests per second: ${YELLOW}$(echo "scale=2; $NUM_REQUESTS / $LOGIN_DURATION" | bc)${NC}"
echo -e "  Success: ${GREEN}$SUCCESS_COUNT${NC} | Failed: ${RED}$FAIL_COUNT${NC}\n"

# Reset counters
SUCCESS_COUNT=0
FAIL_COUNT=0
TOTAL_TIME=0

# Test 2: Get All Users Performance
echo -e "${CYAN}Test 2: Get All Users Endpoint Performance${NC}"
USERS_START=$(date +%s.%N)
for i in $(seq 1 $NUM_REQUESTS); do
    show_progress $i $NUM_REQUESTS
    
    START=$(date +%s.%N)
    HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" -X GET "$BASE_URL/api/users" \
      -H "Authorization: Bearer $TOKEN")
    END=$(date +%s.%N)
    
    REQUEST_TIME=$(echo "$END - $START" | bc)
    TOTAL_TIME=$(echo "$TOTAL_TIME + $REQUEST_TIME" | bc)
    
    if [ "$HTTP_CODE" -eq 200 ]; then
        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
    else
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi
done
USERS_END=$(date +%s.%N)
USERS_DURATION=$(echo "$USERS_END - $USERS_START" | bc)

echo ""
echo -e "${GREEN}✓ Get users test completed${NC}"
echo -e "  Total time: ${YELLOW}${USERS_DURATION}s${NC}"
echo -e "  Average response time: ${YELLOW}$(echo "scale=4; $TOTAL_TIME / $NUM_REQUESTS" | bc)s${NC}"
echo -e "  Requests per second: ${YELLOW}$(echo "scale=2; $NUM_REQUESTS / $USERS_DURATION" | bc)${NC}"
echo -e "  Success: ${GREEN}$SUCCESS_COUNT${NC} | Failed: ${RED}$FAIL_COUNT${NC}\n"

# Reset counters
SUCCESS_COUNT=0
FAIL_COUNT=0
TOTAL_TIME=0

# Test 3: Search Products Performance (Dynamic Query)
echo -e "${CYAN}Test 3: Search Products Endpoint Performance (Dynamic Query)${NC}"
SEARCH_START=$(date +%s.%N)
for i in $(seq 1 $NUM_REQUESTS); do
    show_progress $i $NUM_REQUESTS
    
    START=$(date +%s.%N)
    HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" \
      -X GET "$BASE_URL/api/products?name=laptop&min_price=500&max_price=1500" \
      -H "Authorization: Bearer $TOKEN")
    END=$(date +%s.%N)
    
    REQUEST_TIME=$(echo "$END - $START" | bc)
    TOTAL_TIME=$(echo "$TOTAL_TIME + $REQUEST_TIME" | bc)
    
    if [ "$HTTP_CODE" -eq 200 ]; then
        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
    else
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi
done
SEARCH_END=$(date +%s.%N)
SEARCH_DURATION=$(echo "$SEARCH_END - $SEARCH_START" | bc)

echo ""
echo -e "${GREEN}✓ Search products test completed${NC}"
echo -e "  Total time: ${YELLOW}${SEARCH_DURATION}s${NC}"
echo -e "  Average response time: ${YELLOW}$(echo "scale=4; $TOTAL_TIME / $NUM_REQUESTS" | bc)s${NC}"
echo -e "  Requests per second: ${YELLOW}$(echo "scale=2; $NUM_REQUESTS / $SEARCH_DURATION" | bc)${NC}"
echo -e "  Success: ${GREEN}$SUCCESS_COUNT${NC} | Failed: ${RED}$FAIL_COUNT${NC}\n"

# Reset counters
SUCCESS_COUNT=0
FAIL_COUNT=0
TOTAL_TIME=0

# Test 4: Create Product Performance
echo -e "${CYAN}Test 4: Create Product Endpoint Performance${NC}"
CREATE_START=$(date +%s.%N)
for i in $(seq 1 $NUM_REQUESTS); do
    show_progress $i $NUM_REQUESTS
    
    START=$(date +%s.%N)
    HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$BASE_URL/api/products" \
      -H "Content-Type: application/json" \
      -H "Authorization: Bearer $TOKEN" \
      -d '{
        "name": "Load Test Product '"$i"'",
        "description": "Product created during load test",
        "price": 99.99,
        "stock": 100
      }')
    END=$(date +%s.%N)
    
    REQUEST_TIME=$(echo "$END - $START" | bc)
    TOTAL_TIME=$(echo "$TOTAL_TIME + $REQUEST_TIME" | bc)
    
    if [ "$HTTP_CODE" -eq 201 ]; then
        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
    else
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi
done
CREATE_END=$(date +%s.%N)
CREATE_DURATION=$(echo "$CREATE_END - $CREATE_START" | bc)

echo ""
echo -e "${GREEN}✓ Create product test completed${NC}"
echo -e "  Total time: ${YELLOW}${CREATE_DURATION}s${NC}"
echo -e "  Average response time: ${YELLOW}$(echo "scale=4; $TOTAL_TIME / $NUM_REQUESTS" | bc)s${NC}"
echo -e "  Requests per second: ${YELLOW}$(echo "scale=2; $NUM_REQUESTS / $CREATE_DURATION" | bc)${NC}"
echo -e "  Success: ${GREEN}$SUCCESS_COUNT${NC} | Failed: ${RED}$FAIL_COUNT${NC}\n"

# Summary
echo -e "${BLUE}════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}LOAD TEST SUMMARY${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════${NC}\n"

TOTAL_REQUESTS=$((NUM_REQUESTS * 4))
TOTAL_DURATION=$(echo "$LOGIN_DURATION + $USERS_DURATION + $SEARCH_DURATION + $CREATE_DURATION" | bc)
OVERALL_RPS=$(echo "scale=2; $TOTAL_REQUESTS / $TOTAL_DURATION" | bc)

echo -e "${YELLOW}Total Requests:${NC} $TOTAL_REQUESTS"
echo -e "${YELLOW}Total Duration:${NC} ${TOTAL_DURATION}s"
echo -e "${YELLOW}Overall RPS:${NC} $OVERALL_RPS requests/second\n"

echo -e "${CYAN}Performance by Endpoint:${NC}"
echo -e "  1. Login:          $(echo "scale=2; $NUM_REQUESTS / $LOGIN_DURATION" | bc) req/s"
echo -e "  2. Get Users:      $(echo "scale=2; $NUM_REQUESTS / $USERS_DURATION" | bc) req/s"
echo -e "  3. Search Products:$(echo "scale=2; $NUM_REQUESTS / $SEARCH_DURATION" | bc) req/s"
echo -e "  4. Create Product: $(echo "scale=2; $NUM_REQUESTS / $CREATE_DURATION" | bc) req/s\n"

echo -e "${GREEN}✓ Load test completed successfully!${NC}\n"

# Cleanup recommendation
echo -e "${YELLOW}Note:${NC} This test created $NUM_REQUESTS products."
echo -e "You may want to clean up the test data from the database.\n"