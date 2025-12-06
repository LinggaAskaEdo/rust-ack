# 🧪 API Test Scripts Documentation

This directory contains three comprehensive test scripts for the Rust REST API.

## 📁 Available Scripts

### 1. **test_api.sh** - Complete Test Suite
Full comprehensive testing of all API endpoints with detailed output.

**Features:**
- ✅ 23 different test scenarios
- ✅ Tests all CRUD operations
- ✅ Dynamic query testing (MyBatis-style)
- ✅ Authentication flow testing
- ✅ Middleware validation testing
- ✅ Edge case testing
- ✅ Colored output with status indicators
- ✅ JSON formatting (requires Python3)

**Usage:**
```bash
chmod +x test_api.sh
./test_api.sh
```

**What it tests:**
- Authentication (login/logout)
- User CRUD operations
- Product CRUD operations
- Dynamic SQL queries with filters
- Token validation
- Authorization middleware
- Error handling
- Edge cases (invalid UUIDs, non-existent resources)

---

### 2. **quick_test.sh** - Quick Smoke Test
Fast test for the most common operations.

**Features:**
- ✅ Quick validation of core functionality
- ✅ Takes ~10 seconds to complete
- ✅ Perfect for CI/CD pipelines
- ✅ Tests critical paths only

**Usage:**
```bash
chmod +x quick_test.sh
./quick_test.sh
```

**What it tests:**
1. Login authentication
2. Get all users
3. Create product
4. Search products (dynamic query)
5. Get product by ID
6. Update product
7. Logout
8. Verify unauthorized access after logout

---

### 3. **load_test.sh** - Performance Testing
Load testing tool to measure API performance.

**Features:**
- ✅ Configurable number of requests
- ✅ Multiple endpoint testing
- ✅ Performance metrics (RPS, response time)
- ✅ Progress bar visualization
- ✅ Summary statistics

**Usage:**
```bash
chmod +x load_test.sh

# Run with default 50 requests per endpoint
./load_test.sh

# Run with custom number of requests
./load_test.sh 100
./load_test.sh 500
```

**What it tests:**
1. Login endpoint performance
2. Get users endpoint performance
3. Search products performance (dynamic query)
4. Create product performance

**Metrics provided:**
- Total duration per test
- Average response time
- Requests per second (RPS)
- Success/failure counts
- Overall performance summary

---

## 🚀 Quick Start

### Prerequisites
- Server must be running (`cargo run`)
- PostgreSQL and Redis must be running (`docker-compose up -d`)
- `curl` installed
- (Optional) `python3` for JSON formatting
- `bc` calculator for load testing

### Run All Tests
```bash
# Make scripts executable
chmod +x test_api.sh quick_test.sh load_test.sh

# Run comprehensive test
./test_api.sh

# Run quick smoke test
./quick_test.sh

# Run load test with 100 requests per endpoint
./load_test.sh 100
```

---

## 📊 Test Output Examples

### test_api.sh Output
```
╔════════════════════════════════════════════════════╗
║     Rust REST API - Comprehensive Test Suite      ║
╚════════════════════════════════════════════════════╝

═══════════════════════════════════════════════
Checking Server Status
═══════════════════════════════════════════════

✓ Server is running at http://localhost:8080

═══════════════════════════════════════════════
TEST 1: Login with Valid Credentials
═══════════════════════════════════════════════

Request: POST /api/auth/login
Payload: {"username": "admin", "password": "password123"}
Response (HTTP 200):
{
    "token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
    "expires_in": 86400
}
✓ Login successful! Token received.
```

### quick_test.sh Output
```
Starting Quick API Test...

1. Logging in...
✓ Logged in successfully
Token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...

2. Getting all users...
[
    {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "username": "admin",
        "email": "admin@example.com"
    }
]
✓ Retrieved users
```

### load_test.sh Output
```
╔════════════════════════════════════════════════════╗
║           API Load Testing Tool                    ║
╚════════════════════════════════════════════════════╝

✓ Server is running

Getting authentication token...
✓ Authenticated successfully

════════════════════════════════════════════════════
Starting load test with 100 requests...
════════════════════════════════════════════════════

Test 1: Login Endpoint Performance
Progress: [██████████████████████████████████████████████████] 100% (100/100)
✓ Login test completed
  Total time: 5.32s
  Average response time: 0.0532s
  Requests per second: 18.80
  Success: 100 | Failed: 0

LOAD TEST SUMMARY
════════════════════════════════════════════════════

Total Requests: 400
Total Duration: 22.45s
Overall RPS: 17.82 requests/second
```

---

## 🎯 Test Coverage

### Authentication Tests
- ✅ Valid login
- ✅ Invalid credentials
- ✅ Missing token
- ✅ Invalid token
- ✅ Expired token (after logout)

### User CRUD Tests
- ✅ Create user
- ✅ Get all users
- ✅ Get user by ID
- ✅ Update user
- ✅ Delete user
- ✅ Invalid UUID handling

### Product CRUD Tests
- ✅ Create product
- ✅ Get all products
- ✅ Get product by ID
- ✅ Update product
- ✅ Delete product
- ✅ Verify deletion

### Dynamic Query Tests
- ✅ Search by name (ILIKE)
- ✅ Search by price range (min/max)
- ✅ Search by minimum stock
- ✅ Multiple filters combined
- ✅ Empty result sets

### Security Tests
- ✅ Middleware token validation
- ✅ Unauthorized access attempts
- ✅ Token invalidation on logout
- ✅ Public vs protected endpoints

---

## 🔧 Customization

### Modify Base URL
```bash
# In any script, change this line:
BASE_URL="http://localhost:8080"

# To your custom URL:
BASE_URL="https://api.example.com"
```

### Add Custom Tests
Edit `test_api.sh` and add a new test function:

```bash
test_custom_feature() {
    print_header "TEST XX: Your Custom Test"
    
    api_call "GET" "/api/your-endpoint" "" "$TOKEN"
    
    print_success "Custom test completed"
}

# Add to main() function
main() {
    # ... existing tests ...
    test_custom_feature
}
```

### Adjust Load Test Parameters
```bash
# Change number of requests
./load_test.sh 1000

# Or modify the script default:
NUM_REQUESTS=${1:-100}  # Changed from 50 to 100
```

---

## 📈 Performance Benchmarks

Expected performance on a standard development machine:

| Endpoint | Expected RPS | Notes |
|----------|--------------|-------|
| Login | 15-25 | CPU intensive (bcrypt) |
| Get Users | 50-100 | Simple query |
| Search Products | 40-80 | Dynamic SQL |
| Create Product | 30-60 | Write operation |

**Note:** Performance varies based on:
- Hardware specifications
- Database configuration
- Network latency
- Concurrent load

---

## 🐛 Troubleshooting

### Server Not Running
```
❌ Server is not running at http://localhost:8080
Please start the server with: cargo run
```
**Solution:** Start the server with `cargo run`

### JSON Formatting Not Available
```
ℹ Python3 not found. JSON output will not be formatted.
```
**Solution:** Install Python3 or ignore (doesn't affect tests)

### Permission Denied
```
bash: ./test_api.sh: Permission denied
```
**Solution:** Make script executable
```bash
chmod +x test_api.sh quick_test.sh load_test.sh
```

### bc: command not found (load test)
**Solution:** Install bc calculator
```bash
# Ubuntu/Debian
sudo apt-get install bc

# macOS
brew install bc
```

### Connection Refused
```
curl: (7) Failed to connect to localhost port 8080: Connection refused
```
**Solution:** 
1. Check if server is running: `ps aux | grep rust-rest-api`
2. Check if port is available: `lsof -i :8080`
3. Start databases: `docker-compose up -d`

---

## 📝 Best Practices

### Before Running Tests
1. ✅ Start PostgreSQL and Redis
2. ✅ Start the Rust server
3. ✅ Verify server is responding
4. ✅ Check database has sample data

### During Testing
1. Monitor server logs for errors
2. Watch for memory leaks during load tests
3. Check database connections

### After Testing
1. Review test results
2. Clean up test data if needed
3. Check for any failed tests
4. Monitor system resources

---

## 🔗 Integration with CI/CD

### GitHub Actions Example
```yaml
name: API Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Start services
        run: docker-compose up -d
      
      - name: Build and run server
        run: |
          cargo build --release
          ./target/release/rust-rest-api &
          sleep 5
      
      - name: Run tests
        run: |
          chmod +x quick_test.sh
          ./quick_test.sh
```

### GitLab CI Example
```yaml
test:
  script:
    - docker-compose up -d
    - cargo run &
    - sleep 5
    - chmod +x quick_test.sh
    - ./quick_test.sh
```

---

## 📚 Additional Resources

- [Main API Documentation](../README.md)
- [Cargo.toml Dependencies](../Cargo.toml)
- [Database Schema](../schema.sql)
- [Docker Configuration](../docker-compose.yml)

---

## 🤝 Contributing

To add new tests:
1. Fork the repository
2. Add your test function
3. Update this documentation
4. Submit a pull request

---

## 📄 License

MIT License - See main project LICENSE file