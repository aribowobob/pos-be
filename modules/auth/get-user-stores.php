<?php
function getUserStores($pdo, $userId) {
    $stmt = $pdo->prepare("SELECT stores.id, stores.name, stores.initial FROM users, stores, user_stores WHERE users.id = user_stores.user_id AND user_stores.store_id = stores.id AND users.id = :userId");
    $stmt->execute(['userId' => $userId]);
    return $stmt->fetchAll();
}
?>