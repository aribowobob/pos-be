<?php
ob_start();
require 'cors.php';

// Menyertakan file koneksi database
require 'db.php';
require 'response.php';
require 'modules/auth/get-token.php';
require 'modules/auth/get-user-data-by-token.php';

// Menyertakan file update sales cart item
require 'modules/cart/edit-sales-cart-item.php';

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
    
    $userId = $userData['id'];
    
    if (!isset($input['id']) || !isset($input['qty'])) {
        http_response_code(400);
        sendResponse(400, 'Bad request', null, 'Missing parameter');
        exit;
    }
    
    $cartId = intval($input['id']);
    $qty = intval($input['qty']);
    
    if ($cartId < 1 || $qty < 0) {
        http_response_code(400);
        sendResponse(400, 'Bad request', null, 'Incorrect parameter value');
        exit;
    }
    
    $isCartUpdated = editSalesCartItem($pdo, $cartId, $qty);
    
    if ($isCartUpdated) {
        http_response_code(200);
        sendResponse(200, 'Success', $isCartUpdated, null);
    } else {
        http_response_code(500); // Gagal update
        sendResponse(500, 'Error', null, 'Failed to update sales cart');
    }
} else {
    http_response_code(404);
    sendResponse(404, 'Not found', null, 'Incorrect request method');
}
ob_end_flush();
?>
