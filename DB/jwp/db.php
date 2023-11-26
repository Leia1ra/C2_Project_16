<?php
session_start();
$conn = mysqli_connect("localhost", "root", "c2p16", "movie");
$conn->set_charset("utf8");
if (!$conn) {
    die("데이터베이스 연결 실패: " . mysqli_connect_error());
}
 ?>
