<?php
ob_start();
require 'cors.php';

// Menyertakan file koneksi database
require 'db.php';
require 'response.php';
require 'modules/auth/get-token.php';
require 'modules/auth/get-user-data-by-token.php';

// Menyertakan file untuk update data produk
require 'modules/product/update-product-by-id.php';

if ($_SERVER['REQUEST_METHOD'] === 'PUT') {
    // Jika formatnya JSON, gunakan json_decode
    $input = json_decode(file_get_contents("php://input"), true);
    
    if (json_last_error() !== JSON_ERROR_NONE) {
        http_response_code(400);
        sendResponse(400, 'Bad request', null, 'Invalid JSON format');
        exit;
    }
    
    $token = getToken();

    if (!$token || $token == 401) {
        http_response_code(401);
        sendResponse(401, 'Unauthorized', null, 'Unauthorized');
        exit;
    }
    
    $userData = getUserDataByToken($pdo, $token);
    
    if (!$userData) {
        http_response_code(401);
        sendResponse(401, 'Unauthorized', null, 'Unauthorized');
        exit;
    }
    
    if (!isset($input['id']) || !isset($input['sku']) || !isset($input['name']) || !isset($input['purchase_price']) || !isset($input['sale_price']) || !isset($input['unit_name'])) {
        http_response_code(400);
        sendResponse(400, 'Bad request', null, 'Missing parameter');
        exit;
    }
    
    $id = $input['id'];
    $sku = $input['sku'];
    $name = $input['name'];
    $purchasePrice = $input['purchase_price'];
    $salePrice = $input['sale_price'];
    $unitName = $input['unit_name'];
    
    $isUpdated = updateProductById($pdo, $id, $sku, $name, $purchasePrice, $salePrice, $unitName);
    
    http_response_code(200);
    sendResponse(200, 'Success', $isUpdated, null);
} else {
    
}

ob_end_flush();
?>