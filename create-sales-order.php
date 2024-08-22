<?php
ob_start();
require 'cors.php';

// Menyertakan file koneksi database
require 'db.php';
require 'response.php';
require 'modules/auth/get-token.php';
require 'modules/auth/get-user-data-by-token.php';

if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    $token = getToken();
    
    if (!$token || $token == 401) {
        http_response_code(401);
        sendResponse(401, 'Unauthorized', null, 'Unauthorized');
        exit;
    }
    
    $userData = getUserDataByToken($pdo, $token);
    
    if (!$userData) {
        http_response_code(401);
        sendResponse(401, 'Unauthorized', null, 'Unauthorized');
        exit;
    }
    
    $payload = json_decode(file_get_contents('php://input'), true);
    $userId = $userData['id'];
    $orderNumber = uniqid('TRJ');
    $storeId = isset($payload['store_id']) ? intval($payload['store_id']) : 0;
    $paymentCash = isset($payload['payment_cash']) ? intval($payload['payment_cash']) : 0;
    $paymentNonCash = isset($payload['payment_non_cash']) ? intval($payload['payment_non_cash']) : 0;
    $date = $payload['date'] ?? null;
    
    if (!($userId && $storeId && ($paymentCash + $paymentNonCash > 0) && $date)) {
        http_response_code(400);
        sendResponse(400, 'Bad request', null, 'Bad request');
        exit;
    }
    
    try {
        // Memulai transaksi
        $pdo->beginTransaction();
        
        // Ambil semua item dari sales_cart
        $stmt = $pdo->prepare("
            SELECT * FROM sales_cart WHERE user_id = :user_id AND store_id = :store_id
        ");
        $stmt->bindParam(':user_id', $userId, PDO::PARAM_INT);
        $stmt->bindParam(':store_id', $storeId, PDO::PARAM_INT);
        $stmt->execute();
        $cartItems = $stmt->fetchAll(PDO::FETCH_ASSOC);
        
        if (!$cartItems) {
            throw new Exception('No items in cart.');
        }
        
        // Hitung grand total
        $grandTotal = 0;
        foreach ($cartItems as $item) {
            $grandTotal += $item['sale_price'] * $item['qty'];
        }
        
        $receivable = $grandTotal - $paymentCash - $paymentNonCash;
        
        if ($receivable < 0) {
            $receivable = 0;
        }
        
        // Masukkan data ke sales_orders
        $stmt = $pdo->prepare("
            INSERT INTO sales_orders (order_number, user_id, store_id, date, grand_total, payment_cash, payment_non_cash, receivable, created)
            VALUES (:order_number, :user_id, :store_id, :date, :grand_total, :payment_cash, :payment_non_cash, :receivable, NOW())
        ");
        $stmt->bindParam(':order_number', $orderNumber);
        $stmt->bindParam(':user_id', $userId, PDO::PARAM_INT);
        $stmt->bindParam(':store_id', $storeId, PDO::PARAM_INT);
        $stmt->bindParam(':date', $date, PDO::PARAM_STR);
        $stmt->bindParam(':grand_total', $grandTotal, PDO::PARAM_INT);
        $stmt->bindParam(':payment_cash', $paymentCash, PDO::PARAM_INT);
        $stmt->bindParam(':payment_non_cash', $paymentNonCash, PDO::PARAM_INT);
        $stmt->bindParam(':receivable', $receivable, PDO::PARAM_INT);
        $stmt->execute();
        
        $orderId = $pdo->lastInsertId();
        
        // Masukkan data ke sales_order_details
        $stmt = $pdo->prepare("
            INSERT INTO sales_order_details (order_id, product_id, qty, base_price, discount_type, discount_value, discount_amount, sale_price, total_price)
            VALUES (:order_id, :product_id, :qty, :base_price, :discount_type, :discount_value, :discount_amount, :sale_price, :total_price)
        ");
        
        foreach ($cartItems as $item) {
            $totalPrice = $item['sale_price'] * $item['qty'];
            
            $stmt->bindParam(':order_id', $orderId, PDO::PARAM_INT);
            $stmt->bindParam(':product_id', $item['product_id'], PDO::PARAM_INT);
            $stmt->bindParam(':qty', $item['qty'], PDO::PARAM_INT);
            $stmt->bindParam(':base_price', $item['base_price'], PDO::PARAM_INT);
            $stmt->bindParam(':discount_type', $item['discount_type']);
            $stmt->bindParam(':discount_value', $item['discount_value'], PDO::PARAM_INT);
            $stmt->bindParam(':discount_amount', $item['discount_amount'], PDO::PARAM_INT);
            $stmt->bindParam(':sale_price', $item['sale_price'], PDO::PARAM_INT);
            $stmt->bindParam(':total_price', $totalPrice, PDO::PARAM_INT);
            $stmt->execute();

            // Kurangi stok berdasarkan produk dan toko
            $updateStockStmt = $pdo->prepare("
                UPDATE stock SET qty = qty - :qty WHERE store_id = :store_id AND product_id = :product_id
            ");
            $updateStockStmt->bindParam(':qty', $item['qty'], PDO::PARAM_INT);
            $updateStockStmt->bindParam(':store_id', $storeId, PDO::PARAM_INT);
            $updateStockStmt->bindParam(':product_id', $item['product_id'], PDO::PARAM_INT);
            $updateStockStmt->execute();

            // Periksa apakah pengurangan stok berhasil
            if ($updateStockStmt->rowCount() === 0) {
                throw new Exception('Stock update failed for product_id: ' . $item['product_id']);
            }
        }
        
        // Hapus item dari sales_cart
        $stmt = $pdo->prepare("
            DELETE FROM sales_cart WHERE user_id = :user_id AND store_id = :store_id
        ");
        $stmt->bindParam(':user_id', $userId, PDO::PARAM_INT);
        $stmt->bindParam(':store_id', $storeId, PDO::PARAM_INT);
        $stmt->execute();
        
        // Komit transaksi
        $pdo->commit();
        
        http_response_code(200);
        sendResponse(200, 'Success', ['order_id' => $orderId], null);
    } catch (Exception $e) {
        // Rollback jika ada kesalahan
        $pdo->rollBack();
        http_response_code(500);
        sendResponse(500, 'Internal Server Error', null, $e->getMessage());
    }
} else {
    http_response_code(404);
    sendResponse(404, 'Not found', null, 'Incorrect request method');
}
ob_end_flush();
?>
