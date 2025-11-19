#!/bin/bash

# Test script for Matrix API endpoints
# Replace ESP32_IP with the actual IP address of your ESP32 device

ESP32_IP="192.168.1.100"  # Replace with your ESP32's IP address

echo "Testing Matrix API endpoints..."
echo "ESP32 IP: $ESP32_IP"
echo

# Test 1: Health check
echo "Test 1: Health check"
curl -X GET "http://$ESP32_IP/api/matrix/health"
echo
echo

# Test 2: Status check
echo "Test 2: Status check"
curl -X GET "http://$ESP32_IP/api/matrix/status"
echo
echo

# Test 3: Display text
echo "Test 3: Display text 'HELLO'"
curl -X POST "http://$ESP32_IP/api/matrix/text" \
     -H "Content-Type: application/json" \
     -d '{"text": "HELLO"}'
echo
echo

# Wait a bit to see the text on the matrix
sleep 3

# Test 4: Display custom pattern (smiley face)
echo "Test 4: Display custom pattern (smiley face)"
curl -X POST "http://$ESP32_IP/api/matrix/pattern" \
     -H "Content-Type: application/json" \
     -d '{
           "pattern": [
             [0,0,1,1,1,1,0,0],
             [0,1,0,0,0,0,1,0],
             [1,0,1,0,0,1,0,1],
             [1,0,0,0,0,0,0,1],
             [1,0,1,0,0,1,0,1],
             [1,0,0,1,1,0,0,1],
             [0,1,0,0,0,0,1,0],
             [0,0,1,1,1,1,0,0]
           ]
         }'
echo
echo

# Wait to see the pattern
sleep 3

# Test 5: Clear matrix
echo "Test 5: Clear matrix"
curl -X POST "http://$ESP32_IP/api/matrix/clear"
echo
echo

echo "All tests completed!"