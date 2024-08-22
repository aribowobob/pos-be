<?php
function updateProductById($pdo, $id, $sku, $name, $purchase_price, $sale_price, $unit_name) {
    try {
        // Pastikan $id adalah bilangan bulat
        $id = (int) $id;

        // Query untuk mengupdate data di tabel products
        $sql = "UPDATE products 
                SET sku = :sku, 
                    name = :name, 
                    purchase_price = :purchase_price, 
                    sale_price = :sale_price, 
                    unit_name = :unit_name, 
                    updated = NOW() 
                WHERE id = :id";
        
        $stmt = $pdo->prepare($sql);
        
        // Bind parameter ke query
        $stmt->bindParam(':sku', $sku, PDO::PARAM_STR);
        $stmt->bindParam(':name', $name, PDO::PARAM_STR);
        $stmt->bindParam(':purchase_price', $purchase_price, PDO::PARAM_INT);
        $stmt->bindParam(':sale_price', $sale_price, PDO::PARAM_INT);
        $stmt->bindParam(':unit_name', $unit_name, PDO::PARAM_STR);
        $stmt->bindParam(':id', $id, PDO::PARAM_INT);

        // Eksekusi query
        if ($stmt->execute()) {
            // Mengembalikan true jika berhasil memperbarui data
            return true;
        } else {
            // Mengembalikan false jika gagal memperbarui data
            return false;
        }
    } catch (Exception $e) {
        // Mengembalikan false jika terjadi kesalahan
        error_log($e->getMessage());
        return false;
    }
}
?>
