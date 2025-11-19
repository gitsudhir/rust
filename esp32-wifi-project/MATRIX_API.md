# Matrix API Documentation

This document describes the REST API endpoints available for controlling the MAX7219 8x8 LED matrix connected to the ESP32 device.

## Base URL

All endpoints are relative to the base URL of your ESP32 device:
```
http://<ESP32_IP_ADDRESS>
```

## Endpoints

### 1. Health Check
Check if the matrix is available.

**Endpoint:** `GET /api/matrix/health`  
**Response:**
```json
{
  "status": "available" | "unavailable"
}
```

### 2. Matrix Status
Get detailed status information about the matrix.

**Endpoint:** `GET /api/matrix/status`  
**Response:**
```json
{
  "status": "connected" | "disconnected" | "unavailable",
  "type": "MAX7219_8x8"
}
```

### 3. Display Text
Display text on the matrix.

**Endpoint:** `POST /api/matrix/text`  
**Headers:** `Content-Type: application/json`  
**Body:**
```json
{
  "text": "YOUR_TEXT_HERE"
}
```
**Response:**
```json
{
  "status": "success" | "error",
  "message": "Error description (if status is error)"
}
```

### 4. Display Custom Pattern
Display a custom 8x8 binary pattern on the matrix.

**Endpoint:** `POST /api/matrix/pattern`  
**Headers:** `Content-Type: application/json`  
**Body:**
```json
{
  "pattern": [
    [0,1,0,1,0,1,0,1],
    [1,0,1,0,1,0,1,0],
    [0,1,0,1,0,1,0,1],
    [1,0,1,0,1,0,1,0],
    [0,1,0,1,0,1,0,1],
    [1,0,1,0,1,0,1,0],
    [0,1,0,1,0,1,0,1],
    [1,0,1,0,1,0,1,0]
  ]
}
```
**Response:**
```json
{
  "status": "success" | "error",
  "message": "Error description (if status is error)"
}
```

### 5. Clear Matrix
Clear the matrix (turn off all LEDs).

**Endpoint:** `POST /api/matrix/clear`  
**Response:**
```json
{
  "status": "success" | "error",
  "message": "Error description (if status is error)"
}
```

## CORS Support

All endpoints support CORS (Cross-Origin Resource Sharing) and can be accessed from web browsers.

## Error Handling

All endpoints return appropriate HTTP status codes:
- `200 OK` - Request successful
- `400 Bad Request` - Invalid request data
- `500 Internal Server Error` - Server-side error

Error responses follow this format:
```json
{
  "status": "error",
  "message": "Description of the error"
}
```

## Example Usage

### Display "HELLO" on the matrix:
```bash
curl -X POST "http://192.168.1.100/api/matrix/text" \
     -H "Content-Type: application/json" \
     -d '{"text": "HELLO"}'
```

### Display a smiley face:
```bash
curl -X POST "http://192.168.1.100/api/matrix/pattern" \
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
```

### Clear the matrix:
```bash
curl -X POST "http://192.168.1.100/api/matrix/clear"
```