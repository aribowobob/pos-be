<?php
ob_start();
require 'cors.php';

// Menyertakan file koneksi database
require 'db.php';
require 'response.php';
require 'modules/auth/get-token.php';
require 'modules/auth/get-user-data-by-token.php';

// Menyertakan fungsi untuk menghapus data produk
require 'modules/product/delete-product-by-id.php';

if ($_SERVER['REQUEST_METHOD'] === 'DELETE') {
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
    
    // Cari product_id
    if (!isset($input['id'])) {
        http_response_code(400);
        sendResponse(400, 'Bad request', null, 'Missing parameter');
        exit;
    }
    
    $productId = intval($input['id']);
    
    if ($productId < 1) {
        http_response_code(400);
        sendResponse(400, 'Bad request', null, 'Incorrect parameter value');
        exit;
    }
    
    $isDeleted = softDeleteProductById($pdo, $productId);
    
    if ($isDeleted) {
        http_response_code(200);
        sendResponse(200, 'Success', $isDeleted, null);
    } else {
        http_response_code(500);
        sendResponse(500, 'Internal Server Error', null, 'Failed to delete product');
    }
} else {
    http_response_code(404);
    sendResponse(404, 'Not found', null, 'Incorrect request method');
}
ob_end_flush();
?>