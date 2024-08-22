<?php
function insertNewProduct($pdo, $sku, $name, $purchase_price, $sale_price, $company_id, $unit_name) {
    try {
        // Query untuk menambahkan data ke tabel products
        $sql = "INSERT INTO products (sku, name, purchase_price, sale_price, company_id, unit_name, created, updated)
                VALUES (:sku, :name, :purchase_price, :sale_price, :company_id, :unit_name, NOW(), NOW())";
        
        $stmt = $pdo->prepare($sql);
        
        // Bind parameter ke query
        $stmt->bindParam(':sku', $sku, PDO::PARAM_STR);
        $stmt->bindParam(':name', $name, PDO::PARAM_STR);
        $stmt->bindParam(':purchase_price', $purchase_price, PDO::PARAM_INT);
        $stmt->bindParam(':sale_price', $sale_price, PDO::PARAM_INT);
        $stmt->bindParam(':company_id', $company_id, PDO::PARAM_INT);
        $stmt->bindParam(':unit_name', $unit_name, PDO::PARAM_STR);

        // Eksekusi query
        if ($stmt->execute()) {
            // Mendapatkan ID terakhir yang disisipkan
            $lastId = $pdo->lastInsertId();
            
            // Mengembalikan hasil jika berhasil
            return $lastId;
        } else {
            throw new Exception("Failed to execute statement.");
        }
    } catch (Exception $e) {
        // Log pesan kesalahan
        error_log($e->getMessage());

        // Mengembalikan hasil jika terjadi kesalahan
        return 0;
    }
}
?>
