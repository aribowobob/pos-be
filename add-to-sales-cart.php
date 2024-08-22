<?php
ob_start();
require 'cors.php';

// Menyertakan file koneksi database
require 'db.php';
require 'response.php';
require 'modules/auth/get-token.php';
require 'modules/auth/get-user-data-by-token.php';

// Menyertakan file untuk dapatkan data produk by id
require 'modules/product/find-product-by-id.php';

// Menyertakan file untuk insert/update data ke sales-cart
require 'modules/cart/upsert-item-to-sales-cart.php';

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
$payload = json_decode(file_get_contents('php://input'), true);

if (!isset($payload['store']) || !isset($payload['product']) || !isset($payload['qty'])) {
    http_response_code(400);
    sendResponse(400, 'Bad request', null, 'Missing parameter');
    exit;
}

$storeId = intval($payload['store']);
$productId = intval($payload['product']);
$qty = intval($payload['qty']);

if ($storeId < 1 || $productId < 1 || $qty < 0) {
    http_response_code(400);
    sendResponse(400, 'Bad request', null, 'Incorrect parameter value');
    exit;
}

$productData = findProductById($pdo, $productId);

if (!$productData) {
    http_response_code(400);
    sendResponse(400, 'Bad request', null, 'Missing product');
    exit;
}

$basePrice = $productData['sale_price'];
$lastInsertedId = upsertToSalesCart($pdo, $userId, $storeId, $productId, $qty, $basePrice, 'FIXED', 0);

if (!$lastInsertedId) {
    http_response_code(500);
    sendResponse(500, 'System error', null, 'Failed to insert data');
    exit;
}

http_response_code(200);
sendResponse(200, 'Success', $lastInsertedId > 0, null);

ob_end_flush();
?>