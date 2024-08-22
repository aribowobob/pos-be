<?php
ob_start();
require 'cors.php';

// Menyertakan file koneksi database
require 'db.php';
require 'response.php';
require 'modules/auth/get-token.php';
require 'modules/auth/get-user-data-by-token.php';
require 'modules/auth/get-user-stores.php';

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
$userStores = getUserStores($pdo, $userId);
$response = array(
    'id' => $userData['id'],
    'fullName' => $userData['full_name'],
    'initial' => $userData['initial'],
    'email' => $userData['email'],
    'companyId' => $userData['company_id'],
    'companyName' => $userData['name'],
    'userStores' => is_array($userStores) ? $userStores : null,
    'store' => (is_array($userStores) && count($userStores) > 0) ? $userStores[0] : null,
);
    
// Mengembalikan respons dengan data user
http_response_code(200);
sendResponse(200, 'Success', $response, null);

ob_end_flush();
?>