<?php
ob_start();
require 'cors.php';

// Menyertakan file koneksi database
require 'db.php';
require 'response.php';
require 'modules/auth/get-token.php';
require 'modules/auth/get-user-data-by-token.php';

// Menyertakan fungsi untuk mencari produk dengan id
require 'modules/product/find-product-by-id.php';

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

$id = $_GET['id'];

// Validasi apakah $id adalah bilangan bulat positif
if (isset($_GET['id']) && ctype_digit($id)) {
    $productId = (int)$id;
    
    $product = findProductById($pdo, $productId);
    
    http_response_code(200);
    sendResponse(200, 'Success', $product, null);
} else {
    http_response_code(404);
    sendResponse(404, 'Not found', null, 'Product not found');
}
ob_end_flush();
?>