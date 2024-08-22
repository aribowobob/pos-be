<?php
header('Content-Type: application/json');

// Fungsi untuk mengirim respons JSON
function sendResponse($code, $message, $data = null, $error = null) {
    $response = array(
        'code' => $code,
        'message' => $message,
        'data' => $data,
        'error' => $error
    );

    echo json_encode($response);
}
?>