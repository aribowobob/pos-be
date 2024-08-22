<?php
function deleteSalesCartItem($pdo, $cartId) {
    try {
        $sql = "DELETE FROM sales_cart WHERE id = :id";
        $stmt = $pdo->prepare($sql);
        $stmt->bindParam(':id', $cartId, PDO::PARAM_INT);
        return $stmt->execute();
    } catch (PDOException $e) {
        error_log('Error: ' . $e->getMessage());
        return false;
    }
}
?>
