<?php
function getToken() {
    $headers = getallheaders();

    if (!isset($headers['Authorization'])) {
        return 401;
    }

    $token = str_replace('Bearer ', '', $headers['Authorization']);
    
    return $token;
}
?>