<?php
ob_start();
require 'cors.php';

// Menyertakan file koneksi database
require 'db.php';
require 'response.php';
require 'modules/auth/get-token.php';
require 'modules/auth/get-user-data-by-token.php';

// Menyertakan fungsi untuk mencari produk & stok
require 'modules/product/search-products.php';

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
$keyword = $_GET['keyword'];

if (isset($_GET['store'])) {
    $storeId = intval($_GET['store']);

    if ($storeId < 1) {
        http_response_code(400);
        sendResponse(400, 'Bad request', null, 'Missing parameter');
        exit;
    }
    
    $products = searchProducts($pdo, $keyword, $storeId);
    
    http_response_code(200);
    sendResponse(200, 'Success', $products, null);
} else {
    $products = searchCompanyProducts($pdo, $keyword, $companyId);
    
    http_response_code(200);
    sendResponse(200, 'Success', $products, null);
}

ob_end_flush();
?>