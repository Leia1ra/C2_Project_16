<?php
include "db.php";

$name = $_POST['name'];
$genre = $_POST['genre'];
$category = $_POST['category'];
$year = $_POST['year'];
$grade = $_POST['grade'];
$age = $_POST['age'];
$ott = $_POST['ott'];

$result = mysqli_query($conn,"INSERT INTO ott_table
VALUES('".$name."', '".$genre."', '".$category."', '".$year."', '".$grade."','".$age."', '".$ott."')");

if($result === false){
  echo 'ERROR';
  error_log(mysqli_error($conn));
} else{
echo '등록되었습니다. <a href="index.html">처음으로</a>';
}
?>