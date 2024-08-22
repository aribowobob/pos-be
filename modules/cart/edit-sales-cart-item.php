<?php
function editSalesCartItem($pdo, $id, $qty) {
    try {
        $sql = "
            UPDATE sales_cart 
            SET
                qty = :qty,
                updated = NOW() 
            WHERE id = :id
        ";
    
        $stmt = $pdo->prepare($sql);
        $stmt->bindParam(':id', $id, PDO::PARAM_INT);
        $stmt->bindParam(':qty', $qty, PDO::PARAM_INT);
    
        // Mengeksekusi query
        if ($stmt->execute()) {
            // Memeriksa apakah ada baris yang diperbarui
            if ($stmt->rowCount() > 0) {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    } catch (PDOException $e) {
        echo 'Error: ' . $e->getMessage();
        return false;
    }
}
?>