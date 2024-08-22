<?php
ob_start();
require 'cors.php';

// Menyertakan file koneksi database
require 'db.php';

// Fungsi untuk mendapatkan data user dari Google
function getUserData($token) {
    $url = "https://www.googleapis.com/oauth2/v1/userinfo?access_token=" . $token;
    $ch = curl_init();
    curl_setopt($ch, CURLOPT_URL, $url);
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, 1);
    $response = curl_exec($ch);
    if (curl_errno($ch)) {
        curl_close($ch);
        return null;
    }
    curl_close($ch);
    return json_decode($response, true);
}

// Fungsi untuk memeriksa apakah email ada di dalam database
function checkEmailInDatabase($pdo, $email) {
    $stmt = $pdo->prepare("SELECT id, email FROM users WHERE email = :email");
    $stmt->execute(['email' => $email]);
    return $stmt->fetch();
}

// Fungsi untuk generate token
function generateUserToken($email) {
    $timestamp = time();
    $arr = [$email, $timestamp];
    $emailTime = implode("-", $arr);
    $token = sha1($emailTime);
    
    return $token;
}

// Fungsi untuk insert data ke tabel tokens;
function insertTokens($pdo, $token, $userId) {
    // Menyiapkan query SQL
    $sql = "INSERT INTO tokens (token, user_id, expired) VALUES (:token, :user_id, (DATE_ADD(NOW(), INTERVAL 4 HOUR)))";

    // Menggunakan prepared statement untuk keamanan
    $stmt = $pdo->prepare($sql);
    $stmt->bindParam(':token', $token);
    $stmt->bindParam(':user_id', $userId);

    // Menjalankan query
    $stmt->execute();
}

// Mendapatkan token dari header request
$headers = getallheaders();
if (!isset($headers['Token'])) {
    http_response_code(400);
    echo json_encode(['error' => 'Authorization header missing']);
    exit;
}

$token = $headers['Token'];

// Mendapatkan data user dari Google
$userData = getUserData($token);
if (!$userData || !isset($userData['email'])) {
    http_response_code(400);
    echo json_encode(['error' => 'Invalid token or user data not found']);
    exit;
}

$email = $userData['email'];

// Memeriksa apakah email ada di dalam database
$user = checkEmailInDatabase($pdo, $email);

if (!$user) {
    http_response_code(401);
    echo json_encode(['error' => 'Unauthorized']);
    exit;
}

// Insert ke tabel tokens
$userToken = generateUserToken($email);

insertTokens($pdo, $userToken, $user['id']);

// Mengembalikan respons dengan data user
http_response_code(200);

require 'response.php';

sendResponse(200, 'Success', $userToken, null);

ob_end_flush();
?>
