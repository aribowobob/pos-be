<?php
ob_start();
require 'cors.php';

// Menyertakan file koneksi database
require 'db.php';
require 'response.php';
require 'modules/auth/get-token.php';
require 'modules/auth/get-user-data-by-token.php';

// Menyertakan file untuk insert data produk
require 'modules/product/insert-new-product.php';

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

$userId = $userData['id'];
$companyId = $userData['company_id'];
$payload = json_decode(file_get_contents('php://input'), true);

if (!isset($payload['sku']) || !isset($payload['name']) || !isset($payload['purchase_price']) || !isset($payload['sale_price']) || !isset($payload['unit_name']) || !$companyId) {
    http_response_code(400);
    sendResponse(400, 'Bad request', null, 'Missing parameter');
    exit;
}

$sku = $payload['sku'];
$name = $payload['name'];
$purchasePrice = $payload['purchase_price'];
$salePrice = $payload['sale_price'];
$unitName = $payload['unit_name'];

$lastInsertedId = insertNewProduct($pdo, $sku, $name, $purchasePrice, $salePrice, $companyId, $unitName);

if ($lastInsertedId <= 0) {
    http_response_code(500);
    sendResponse(500, 'System error', null, 'Failed to insert data');
    exit;
}

http_response_code(200);
sendResponse(200, 'Success', $lastInsertedId > 0, null);

ob_end_flush();
?>