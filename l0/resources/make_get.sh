#!/bin/bash

curl -H "Accept: application/json" -H "Content-Type: application/json" -X GET http://localhost:3000/api/orders/ && echo ""
