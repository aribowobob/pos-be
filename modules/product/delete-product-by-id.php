<?php
function deleteProductById($pdo, $id) {
    try {
        // Pastikan $id adalah bilangan bulat
        $id = (int) $id;

        // Query untuk menghapus data dari tabel products
        $sql = "DELETE FROM products WHERE id = :id";

        $stmt = $pdo->prepare($sql);
        
        // Bind parameter id ke query
        $stmt->bindParam(':id', $id, PDO::PARAM_INT);

        // Eksekusi query
        if ($stmt->execute()) {
            // Mengembalikan true jika berhasil menghapus data
            return true;
        } else {
            // Mengembalikan false jika gagal menghapus data
            return false;
        }
    } catch (Exception $e) {
        // Mengembalikan false jika terjadi kesalahan
        error_log($e->getMessage());
        return false;
    }
}

function softDeleteProductById($pdo, $id) {
    try {
        // Pastikan $id adalah bilangan bulat
        $id = (int) $id;

        // Query untuk mengupdate kolom is_deleted menjadi 1
        $sql = "UPDATE products 
                SET is_deleted = 1, 
                    updated = NOW() 
                WHERE id = :id";
        
        $stmt = $pdo->prepare($sql);
        
        // Bind parameter ke query
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
