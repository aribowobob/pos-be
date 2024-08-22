<?php
function upsertToSalesCart($pdo, $userId, $storeId, $productId, $qty, $basePrice, $discountType, $discountValue) {
    try {
        // Perhitungan harga & diskon
        $discountAmount = $discountValue;
        
        if ($discountType == 'PERCENTAGE') {
            if ($discountValue > 100) {
                $discountValue = 100;
            } else if ($discountValue < 0) {
                $discountValue = 0;
            }
            
            $discountAmount = floor(($discountValue / 100) * $basePrice);
        }
        
        $salePrice = $basePrice - $discountAmount;
        
        // Pertama, cek apakah kombinasi product_id, user_id, dan store_id sudah ada
        $sql = "
            SELECT id, qty 
            FROM sales_cart 
            WHERE product_id = :productId AND user_id = :userId AND store_id = :storeId
        ";
        $stmt = $pdo->prepare($sql);
        $stmt->bindParam(':productId', $productId, PDO::PARAM_INT);
        $stmt->bindParam(':userId', $userId, PDO::PARAM_INT);
        $stmt->bindParam(':storeId', $storeId, PDO::PARAM_INT);
        $stmt->execute();
        $existingCart = $stmt->fetch(PDO::FETCH_ASSOC);
        
        if ($existingCart) {
            // Jika ada, tambahkan qty baru ke qty yang sudah ada
            $newQty = $existingCart['qty'] + $qty;

            // Update data dengan qty baru
            $sql = "
                UPDATE sales_cart 
                SET qty = :newQty, 
                    base_price = :basePrice,
                    discount_type = :discountType, 
                    discount_value = :discountValue, 
                    discount_amount = :discountAmount, 
                    sale_price = :salePrice, 
                    updated = NOW() 
                WHERE id = :cartId
            ";
            $stmt = $pdo->prepare($sql);
            $stmt->bindParam(':newQty', $newQty, PDO::PARAM_INT);
            $stmt->bindParam(':basePrice', $basePrice, PDO::PARAM_INT);
            $stmt->bindParam(':discountType', $discountType, PDO::PARAM_STR);
            $stmt->bindParam(':discountValue', $discountValue, PDO::PARAM_INT);
            $stmt->bindParam(':discountAmount', $discountAmount, PDO::PARAM_INT);
            $stmt->bindParam(':salePrice', $salePrice, PDO::PARAM_INT);
            $stmt->bindParam(':cartId', $existingCart['id'], PDO::PARAM_INT);
            
            // Mengeksekusi query
            if ($stmt->execute()) {
                // Memeriksa apakah ada baris yang diperbarui
                if ($stmt->rowCount() > 0) {
                    return $existingCart['id'];
                } else {
                    return 0;
                }
            } else {
                return 0;
            }
        } else {
            $sql = "
                INSERT INTO sales_cart (
                    user_id, 
                    store_id, 
                    product_id, 
                    qty, 
                    base_price, 
                    discount_type, 
                    discount_value, 
                    discount_amount, 
                    sale_price, 
                    created, 
                    updated
                ) VALUES (
                    :user_id, 
                    :store_id, 
                    :product_id, 
                    :qty, 
                    :base_price, 
                    :discount_type, 
                    :discount_value, 
                    :discount_amount, 
                    :sale_price, 
                    NOW(), 
                    NOW()
                )
            ";
    
            $stmt = $pdo->prepare($sql);
    
            // Bind parameters
            $stmt->bindParam(':user_id', $userId, PDO::PARAM_INT);
            $stmt->bindParam(':store_id', $storeId, PDO::PARAM_INT);
            $stmt->bindParam(':product_id', $productId, PDO::PARAM_INT);
            $stmt->bindParam(':qty', $qty, PDO::PARAM_INT);
            $stmt->bindParam(':base_price', $basePrice, PDO::PARAM_INT);
            $stmt->bindParam(':discount_type', $discountType, PDO::PARAM_STR);
            $stmt->bindParam(':discount_value', $discountValue, PDO::PARAM_INT);
            $stmt->bindParam(':discount_amount', $discountAmount, PDO::PARAM_INT);
            $stmt->bindParam(':sale_price', $salePrice, PDO::PARAM_INT);
    
            // Execute the query
            $stmt->execute();
    
            // Return the ID of the last inserted row
            return $pdo->lastInsertId();
        }
    } catch (PDOException $e) {
        echo 'Error: ' . $e->getMessage();
        return 0;
    }
}

?>