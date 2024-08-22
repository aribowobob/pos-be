<?php
// Fungsi untuk memeriksa token yang dikirim valid.
function getUserDataByToken($pdo, $token) {
    $stmt = $pdo->prepare("SELECT users.id, users.full_name, users.initial, users.email, companies.id AS company_id, companies.name FROM users, tokens, companies WHERE users.id = tokens.user_id AND users.company_id = companies.id AND token = :token AND expired > NOW()");
    $stmt->execute(['token' => $token]);
    return $stmt->fetch();
}
?>