<?php
function searchProducts($pdo, $keyword, $storeId) {
    try {
        $sql = "
            SELECT 
                products.id,
                products.sku,
                products.name, 
                products.sale_price, 
                products.unit_name, 
                CAST(COALESCE(stock.qty, 0) AS UNSIGNED) AS stock 
            FROM 
                products 
            LEFT JOIN 
                stock ON products.id = stock.product_id AND stock.store_id = :storeId
            WHERE 
                products.name LIKE :keyword
                AND is_deleted = 0
        ";

        // Menambahkan wildcard % di kedua sisi keyword
        $formattedKeyword = "%" . $keyword . "%";
        
        $stmt = $pdo->prepare($sql);
        $stmt->bindParam(':keyword', $formattedKeyword, PDO::PARAM_STR);
        $stmt->bindParam(':storeId', $storeId, PDO::PARAM_INT);
        $stmt->execute();
        
        return $stmt->fetchAll(PDO::FETCH_ASSOC);
    } catch (PDOException $e) {
        echo 'Error: ' . $e->getMessage();
    }
}

function searchCompanyProducts($pdo, $keyword, $companyId) {
    try {
        $sql = "
            SELECT 
                products.id,
                products.sku,
                products.name, 
                products.purchase_price, 
                products.sale_price, 
                products.unit_name
            FROM 
                products
            WHERE 
                products.name LIKE :keyword
                AND company_id = :companyId
                AND is_deleted = 0
        ";

        // Menambahkan wildcard % di kedua sisi keyword
        $formattedKeyword = "%" . $keyword . "%";
        
        $stmt = $pdo->prepare($sql);
        $stmt->bindParam(':keyword', $formattedKeyword, PDO::PARAM_STR);
        $stmt->bindParam(':companyId', $companyId, PDO::PARAM_INT);
        $stmt->execute();
        
        return $stmt->fetchAll(PDO::FETCH_ASSOC);
    } catch (PDOException $e) {
        echo 'Error: ' . $e->getMessage();
    }
}
?>
