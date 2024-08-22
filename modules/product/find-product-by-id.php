<?php
function findProductById($pdo, $productId) {
    try {
        $sql = "
            SELECT 
                id,
                sku,
                name, 
                purchase_price, 
                sale_price, 
                unit_name,
                is_deleted
            FROM 
                products 
            WHERE 
                id = :id
                AND is_deleted = 0
        ";

        $stmt = $pdo->prepare($sql);
        $stmt->bindParam(':id', $productId, PDO::PARAM_INT);
        $stmt->execute();
        
        return $stmt->fetch(PDO::FETCH_ASSOC);
    } catch (PDOException $e) {
        echo 'Error: ' . $e->getMessage();
    }
}
?>
