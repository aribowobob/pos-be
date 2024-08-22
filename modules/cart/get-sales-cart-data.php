<?php
function getSalesCartData($pdo, $userId, $storeId) {
    try {
        $sql = "
            SELECT 
                sales_cart.id, 
                products.name, 
                sales_cart.qty, 
                sales_cart.base_price, 
                sales_cart.discount_type, 
                sales_cart.discount_value, 
                sales_cart.discount_amount, 
                sales_cart.sale_price, 
                CAST(COALESCE(stock.qty, 0) AS UNSIGNED) AS stock 
            FROM 
                sales_cart 
            JOIN 
                products ON sales_cart.product_id = products.id 
            LEFT JOIN 
                stock ON products.id = stock.product_id AND stock.store_id = :storeId1 
            WHERE 
                sales_cart.user_id = :userId 
                AND sales_cart.store_id = :storeId2
        ";

        $stmt = $pdo->prepare($sql);
        $stmt->bindParam(':userId', $userId, PDO::PARAM_INT);
        $stmt->bindParam(':storeId1', $storeId, PDO::PARAM_INT);
        $stmt->bindParam(':storeId2', $storeId, PDO::PARAM_INT);
        $stmt->execute();
        
        return $stmt->fetchAll(PDO::FETCH_ASSOC);
    } catch (PDOException $e) {
        echo 'Error: ' . $e->getMessage();
    }
}
?>
