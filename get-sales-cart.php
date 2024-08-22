<?php
ob_start();
require 'cors.php';

// Menyertakan file koneksi database
require 'db.php';
require 'response.php';
require 'modules/auth/get-token.php';
require 'modules/auth/get-user-data-by-token.php';

require 'modules/cart/get-sales-cart-data.php';

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

if (!isset($_GET['store'])) {
    http_response_code(500);
    sendResponse(500, 'System error', null, 'System error');
    exit;
}

$storeId = intval($_GET['store']);

if ($storeId < 1) {
    http_response_code(500);
    sendResponse(500, 'System error', null, 'System error');
    exit;
}

$cartItems = getSalesCartData($pdo, $userId, $storeId);

http_response_code(200);
sendResponse(200, 'Success', $cartItems, null);

ob_end_flush();
?>